# Shnek
A simple 3D snake game in Rust.

## Project description
This project is a simple 3D snake game written in Rust. The goal of the game is
to guide the snake through 3D space, collect food and avoid colliding
with your own body. There are no walls - the space wraps around:
moving too far to the right brings you back on the left, and the same applies in
all other directions. To make things easier to see all drawing is repeated a 
few times in all directions.

## Run the project
### Precompiled binaries
There are also releases with precompiled binaries for Ubuntu and Windows.
You simply download the zip for your system, unzip it and run `shnek`/`shnek.exe`.
Windows will warn you this program doesn't have a verified publisher but will
probably still let you run it after clicking on more info.

### Compiling from scratch
To compile and run the game (when debugging):
```sh
cargo run
```
and to run more optimized build:
```sh
cargo run --release
```
Optimization level is set to 3 in both cases.

You will need additional build tools for this to work on Windows.
On Linux only `libasound2-dev` should be required.

### Controls
Press `W`, `A`, `S` and `D` to turn up, left, down and right. You can rotate
without changing direction using `Q` and `E`. Pressing `Left Shift` will let
you move faster (just don't press it for too long). You can pause the game
by pressing `Space` or `Esc`.

## Contributing
Before pushing (or at least before making a pull request) run these commands:
```sh
cargo fmt
cargo clippy
```
