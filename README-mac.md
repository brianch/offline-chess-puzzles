# Offline Chess Puzzles on Mac

To build and run on macOS:
- Open terminal.
- Clone the repository: ````git clone [URL to GITHUB repo]````
- Build and run: ````cargo run````
- If build fails, you may need to install Rust, for example:
````
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````

After the initial build, the program can be started from the terminal:
- Open terminal.
- Navigate to the repo folder.
- Run: ````./target/debug/offline-chess-puzzles````

To set up the program to be launchable from Spotlight on Mac:
- Open terminal.
- Navigate to the repo folder.
- Run: ````./mac-app.sh````
- In Spotlight, the program can be found as "Offline Chess Puzzles".