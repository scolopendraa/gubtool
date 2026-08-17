# gubtool

A tui practice tool for soulsborne games. At the moment, it supports all versions of Elden Ring and Dark Souls 2. Runs natively on both linux and windows.

Certain terminal emulators, like KDE konsole, does not properly display the dimming effect of inactive lists.

## Build instructions
The program is built with [cargo](https://rust-lang.org/tools/install/).
`gcc` must be installed and avilable in your `PATH`. On linux, this should be installed automatically when installing cargo. On windows, it can be installed through winget with this command:
```sh
winget install --id BrechtSanders.WinLibs.POSIX.UCRT
```

To build the program, clone this repository and navigate to the directory:

```sh
git clone https://github.com/scolopendraa/gubtool.git
cd gubtool
```

Then build with cargo:
```sh
cargo build --release
```
The compiled executable will be located in `target/release/`.

## Credits
- Thanks a lot to [Shilkey](https://github.com/borgCode) for being very helpful and for his huge amount of work on reverse engineering these games. He hosts a suite of practice tools from which much of the logic for this project is taken.
- [FromSoftware-rs](https://github.com/vswarte/fromsoftware-rs.git), reconstructions of class layouts.
