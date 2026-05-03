<p align="center">
    <img width="200" alt="AcreetionOS Terminal Logo" src="https://raw.githubusercontent.com/acreetionos-terminal/acreetionos-terminal/master/extra/logo/compat/acreetionos-terminal-term%2Bscanlines.png">
</p>

<h1 align="center">AcreetionOS Terminal - A fast, cross-platform, OpenGL terminal emulator</h1>

<p align="center">
  <img alt="AcreetionOS Terminal - A fast, cross-platform, OpenGL terminal emulator"
       src="https://raw.githubusercontent.com/acreetionos-terminal/acreetionos-terminal/master/extra/promo/acreetionos-terminal-readme.png">
</p>

## About

AcreetionOS Terminal is a modern terminal emulator that comes with sensible defaults, but
allows for extensive [configuration](#configuration). By integrating with other
applications, rather than reimplementing their functionality, it manages to
provide a flexible set of [features](./docs/features.md) with high performance.
The supported platforms currently consist of BSD, Linux, macOS and Windows.

The software is considered to be at a **beta** level of readiness; there are
a few missing features and bugs to be fixed, but it is already used by many as
a daily driver.

Precompiled binaries are available from the [GitHub releases page](https://github.com/acreetionos-terminal/acreetionos-terminal/releases).

Join [`#acreetionos-terminal`] on libera.chat if you have questions or looking for a quick help.

[`#acreetionos-terminal`]: https://web.libera.chat/gamja/?channels=#acreetionos-terminal

## Features

You can find an overview over the features available in AcreetionOS Terminal [here](./docs/features.md).

## Further information

- [Announcing AcreetionOS Terminal, a GPU-Accelerated Terminal Emulator](https://jwilm.io/blog/announcing-acreetionos-terminal/) January 6, 2017
- [A talk about AcreetionOS Terminal at the Rust Meetup January 2017](https://www.youtube.com/watch?v=qHOdYO3WUTk) January 19, 2017
- [AcreetionOS Terminal Lands Scrollback, Publishes Benchmarks](https://jwilm.io/blog/acreetionos-terminal-lands-scrollback/) September 17, 2018

## Installation

AcreetionOS Terminal can be installed by using various package managers on Linux, BSD,
macOS and Windows.

Prebuilt binaries for macOS and Windows can also be downloaded from the
[GitHub releases page](https://github.com/acreetionos-terminal/acreetionos-terminal/releases).

For everyone else, the detailed instructions to install AcreetionOS Terminal can be found
[here](INSTALL.md).

### Requirements

- At least OpenGL ES 2.0
- [Windows] ConPTY support (Windows 10 version 1809 or higher)

## Configuration

You can find the documentation for AcreetionOS Terminal's configuration in `man 5
acreetionos-terminal`, or by looking at [the website] if you do not have the manpages
installed.

[the website]: https://acreetionos-terminal.org/config-acreetionos-terminal.html

AcreetionOS Terminal doesn't create the config file for you, but it looks for one in the
following locations:

1. `$XDG_CONFIG_HOME/acreetionos-terminal/acreetionos-terminal.toml`
2. `$XDG_CONFIG_HOME/acreetionos-terminal.toml`
3. `$HOME/.config/acreetionos-terminal/acreetionos-terminal.toml`
4. `$HOME/.acreetionos-terminal.toml`
5. `/etc/acreetionos-terminal/acreetionos-terminal.toml`

On Windows, the config file will be looked for in:

* `%APPDATA%\acreetionos-terminal\acreetionos-terminal.toml`

## Contributing

A guideline about contributing to AcreetionOS Terminal can be found in the
[`CONTRIBUTING.md`](CONTRIBUTING.md) file.

## FAQ

**_Is it really the fastest terminal emulator?_**

Benchmarking terminal emulators is complicated. AcreetionOS Terminal uses
[vtebench](https://github.com/acreetionos-terminal/vtebench) to quantify terminal emulator
throughput and manages to consistently score better than the competition using
it. If you have found an example where this is not the case, please report a
bug.

Other aspects like latency or framerate and frame consistency are more difficult
to quantify. Some terminal emulators also intentionally slow down to save
resources, which might be preferred by some users.

If you have doubts about AcreetionOS Terminal's performance or usability, the best way to
quantify terminal emulators is always to test them with **your** specific
usecases.

**_Why isn't feature X implemented?_**

AcreetionOS Terminal has many great features, but not every feature from every other
terminal. This could be for a number of reasons, but sometimes it's just not a
good fit for AcreetionOS Terminal. This means you won't find things like tabs or splits
(which are best left to a window manager or [terminal multiplexer][tmux]) nor
niceties like a GUI config editor.

[tmux]: https://github.com/tmux/tmux

## License

AcreetionOS Terminal is released under the [Apache License, Version 2.0].

[Apache License, Version 2.0]: https://github.com/acreetionos-terminal/acreetionos-terminal/blob/master/LICENSE-APACHE
