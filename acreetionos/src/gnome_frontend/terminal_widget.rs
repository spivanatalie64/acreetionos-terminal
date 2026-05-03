use std::cell::RefCell;
use std::ffi::CStr;
use std::rc::Rc;
use std::sync::Arc;

use crossfont::{Rasterize, Rasterizer, Size as FontSize};
use gtk4::gdk::Key;
use gtk4::prelude::*;
use gtk4::{EventControllerKey, GLArea, Widget};
use acreetionos_terminal::event::Event as TerminalEvent;
use acreetionos_terminal::event_loop::{EventLoop as PtyEventLoop, EventLoopSender, Msg};
use acreetionos_terminal::grid::Dimensions;
use acreetionos_terminal::index::{Column, Line, Point};
use acreetionos_terminal::sync::FairMutex;
use acreetionos_terminal::term::cell::Flags;
use acreetionos_terminal::term::Term;
use acreetionos_terminal::tty;
use acreetionos_terminal::vte::ansi::{Color, NamedColor};

use crate::config::UiConfig;
use crate::display::color::{self, List, Rgb};
use crate::display::content::RenderableCell;
use crate::display::SizeInfo;
use crate::gnome_frontend::search::SearchBar;
use crate::renderer::{GlyphCache, Renderer};

struct GtkEventProxy;

impl acreetionos_terminal::event::EventListener for GtkEventProxy {
    fn send_event(&self, _event: TerminalEvent) {}
}

struct GtkTerminalState {
    renderer: Renderer,
    glyph_cache: GlyphCache,
    size_info: SizeInfo,
    font_size: FontSize,
    colors: List,
}

pub struct TerminalWidget {
    gl_area: GLArea,
    terminal: Arc<FairMutex<Term<GtkEventProxy>>>,
    sender: Option<EventLoopSender>,
    search_bar: SearchBar,
    state: Rc<RefCell<Option<GtkTerminalState>>>,
    config: Rc<UiConfig>,
}

impl TerminalWidget {
    pub fn new(config: Rc<UiConfig>) -> Self {
        let gl_area = GLArea::new();
        gl_area.set_required_version(3, 3);
        gl_area.set_auto_render(true);
        gl_area.set_has_depth_buffer(false);
        gl_area.set_has_stencil_buffer(false);
        gl_area.set_focusable(true);
        gl_area.set_can_focus(true);
        gl_area.grab_focus();

        let search_bar = SearchBar::new();

        let size_info = SizeInfo::new(800.0, 600.0, 10.0, 20.0, 2.0, 2.0, false);

        let terminal = Arc::new(FairMutex::new(
            Term::new(config.term_options(), &size_info, GtkEventProxy),
        ));

        let pty_config = config.pty_config();
        let window_id: u64 = 0;
        let sender = match tty::new(&pty_config, size_info.into(), window_id.into()) {
            Ok(pty) => {
                match PtyEventLoop::new(
                    Arc::clone(&terminal),
                    GtkEventProxy,
                    pty,
                    pty_config.drain_on_exit,
                    config.debug.ref_test,
                ) {
                    Ok(loop_) => {
                        let tx = loop_.channel();
                        let _thread = loop_.spawn();
                        Some(tx)
                    },
                    Err(err) => {
                        log::error!("Failed to create PTY event loop: {err}");
                        None
                    },
                }
            },
            Err(err) => {
                log::error!("Failed to create PTY: {err}");
                None
            },
        };

        let widget = TerminalWidget {
            gl_area,
            terminal,
            sender,
            search_bar,
            state: Rc::new(RefCell::new(None)),
            config,
        };

        widget.connect_signals();
        widget
    }

    pub fn widget(&self) -> Widget {
        self.gl_area.clone().upcast::<Widget>()
    }

    pub fn connect_signals(&self) {
        let state = self.state.clone();
        let terminal = self.terminal.clone();
        let config = self.config.clone();

        self.gl_area.connect_realize(move |gl_area| {
            gl_area.make_current();

            let scale_factor = gl_area.scale_factor() as f32;
            let rasterizer = match Rasterizer::new() {
                Ok(r) => r,
                Err(e) => {
                    log::error!("Failed to create rasterizer: {e}");
                    return;
                },
            };

            let font_size = config.font.size().scale(scale_factor);
            let font = config.font.clone().with_size(font_size);
            let glyph_cache = match GlyphCache::new(rasterizer, &font) {
                Ok(gc) => gc,
                Err(e) => {
                    log::error!("Failed to create glyph cache: {e}");
                    return;
                },
            };

            let metrics = glyph_cache.font_metrics();
            let cell_width = metrics.average_advance;
            let cell_height = metrics.line_height;
            let padding = config.window.padding(scale_factor);

            let alloc = gl_area.allocation();
            let width = alloc.width() as f32;
            let height = alloc.height() as f32;

            let size_info = SizeInfo::new(
                width as f32, height as f32,
                cell_width as f32, cell_height as f32,
                padding.0, padding.1,
                false,
            );

            let load_fn = |sym: &CStr| -> *const std::ffi::c_void {
                unsafe {
                    libc::dlsym(libc::RTLD_DEFAULT, sym.as_ptr())
                }
            };

            let renderer = match Renderer::new(&load_fn, config.debug.renderer, false) {
                Ok(r) => r,
                Err(e) => {
                    log::error!("Failed to create renderer: {e}");
                    return;
                },
            };

            renderer.resize(&size_info);
            renderer.clear(config.colors.primary.background, config.window_opacity());

            *state.borrow_mut() = Some(GtkTerminalState {
                renderer,
                glyph_cache,
                size_info,
                font_size,
                colors: List::from(&config.colors),
            });
        });

        let state_render = self.state.clone();
        let terminal_render = self.terminal.clone();
        let config_render = self.config.clone();

        self.gl_area.connect_render(move |_gl_area, _context| {
            let mut guard = state_render.borrow_mut();
            let gtk_state = match guard.as_mut() {
                Some(s) => s,
                None => return true.into(),
            };

            let mut term = terminal_render.lock();
            let grid = term.grid();
            let display_offset = grid.display_offset();
            let screen_lines = gtk_state.size_info.screen_lines();
            let num_cols = gtk_state.size_info.columns();
            let colors = &gtk_state.colors;
            let opacity = config_render.window_opacity();

            gtk_state.renderer.clear(colors[NamedColor::Background], opacity);

            let mut cells: Vec<RenderableCell> = Vec::with_capacity(screen_lines * num_cols);

            for line_idx in 0..screen_lines {
                let grid_line =
                    display_offset as i32 + screen_lines as i32 - 1 - line_idx as i32;
                let row = &grid[Line(grid_line)];

                for col_idx in 0..num_cols {
                    let col = Column(col_idx);
                    let cell = &row[col];

                    if cell.c == ' ' && cell.flags.is_empty() {
                        continue;
                    }

                    let fg = cell_color_to_rgb(colors, cell.fg, cell.flags);
                    let bg = cell_color_to_rgb(colors, cell.bg, cell.flags);
                    let bg_alpha = if cell.bg == Color::Named(NamedColor::Background) {
                        opacity
                    } else {
                        1.0
                    };

                    cells.push(RenderableCell {
                        flags: cell.flags,
                        character: cell.c,
                        bg_alpha,
                        point: Point::new(line_idx, col),
                        fg,
                        bg,
                        underline: fg,
                        extra: None,
                    });
                }
            }

            drop(term);

            gtk_state.renderer.draw_cells(
                &gtk_state.size_info,
                &mut gtk_state.glyph_cache,
                cells.into_iter(),
            );

            gtk_state.renderer.finish();

            false.into()
        });

        self.connect_input_signals();
    }

    fn connect_input_signals(&self) {
        let controller = EventControllerKey::new();

        let sender = self.sender.clone();

        controller.connect_key_pressed(move |_controller, keyval, _keycode, state_mod| {
            let sender = match &sender {
                Some(s) => s,
                None => return false.into(),
            };

            let ctrl = state_mod.intersects(gtk4::gdk::ModifierType::CONTROL_MASK);

            if ctrl {
                if let Some(ch) = keyval.to_unicode() {
                    if ch.is_ascii_lowercase() {
                        let code = (ch as u8) - b'a' + 1;
                        let _ = sender.send(Msg::Input(vec![code].into()));
                        return true.into();
                    }
                }
            }

            let seq: Option<&[u8]> = match keyval {
                Key::Return | Key::KP_Enter => Some(b"\r"),
                Key::BackSpace => Some(b"\x7f"),
                Key::Tab => Some(b"\t"),
                Key::Escape => Some(b"\x1b"),
                Key::Up => Some(b"\x1b[A"),
                Key::Down => Some(b"\x1b[B"),
                Key::Right => Some(b"\x1b[C"),
                Key::Left => Some(b"\x1b[D"),
                Key::Page_Up => Some(b"\x1b[5~"),
                Key::Page_Down => Some(b"\x1b[6~"),
                Key::Home => Some(b"\x1b[H"),
                Key::End => Some(b"\x1b[F"),
                Key::Insert => Some(b"\x1b[2~"),
                Key::Delete => Some(b"\x1b[3~"),
                Key::F1 => Some(b"\x1bOP"),
                Key::F2 => Some(b"\x1bOQ"),
                Key::F3 => Some(b"\x1bOR"),
                Key::F4 => Some(b"\x1bOS"),
                Key::F5 => Some(b"\x1b[15~"),
                Key::F6 => Some(b"\x1b[17~"),
                Key::F7 => Some(b"\x1b[18~"),
                Key::F8 => Some(b"\x1b[19~"),
                Key::F9 => Some(b"\x1b[20~"),
                Key::F10 => Some(b"\x1b[21~"),
                Key::F11 => Some(b"\x1b[23~"),
                Key::F12 => Some(b"\x1b[24~"),
                _ => None,
            };

            if let Some(seq) = seq {
                let _ = sender.send(Msg::Input(seq.to_vec().into()));
            }

            false.into()
        });

        self.gl_area.add_controller(controller);
    }

    pub fn toggle_search(&self) {
        self.search_bar.toggle();
    }
}

fn cell_color_to_rgb(colors: &List, color: Color, flags: Flags) -> Rgb {
    match color {
        Color::Named(named) => {
            if flags.contains(Flags::DIM) && named == NamedColor::Foreground {
                colors[NamedColor::DimForeground]
            } else {
                colors[named]
            }
        },
        Color::Spec(rgb) => Rgb::new(rgb.r, rgb.g, rgb.b),
        Color::Indexed(idx) => colors[idx as usize],
    }
}


