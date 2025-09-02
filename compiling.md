
# Compiling

## Ubuntu

Install the dependencies:
````
sudo apt-get install libasound2-dev libgtk-3-dev libsqlite3-dev
````
Compile:
````
cargo build --release
````

## Mac

To build and run on macOS:
- Open terminal.
- Clone the repository: ````git clone https://github.com/brianch/offline-chess-puzzles.git````
- Build and run: ````cargo run --release````
- If build fails, you may need to install Rust, for example:
````
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````

After the initial build, the program can be started from the terminal:
- Open terminal.
- Navigate to the repo folder.
- Run: ````./target/release/offline-chess-puzzles````

To set up the program to be launchable from Spotlight on Mac:
- Open terminal.
- Navigate to the repo folder.
- Run: ````./mac-app.sh````
- In Spotlight, the program can be found as "Offline Chess Puzzles".


