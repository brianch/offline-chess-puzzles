# offline-chess-puzzles
Tool to view and solve puzzles from the lichess puzzle database.

<img src="https://github.com/brianch/offline-chess-puzzles/blob/main/demo.gif" width="200"/>

It's a very simple tool for those who want to practice offline, it has filters by puzzle rating and theme.

I need to thank lichess for creating the puzzle database (https://database.lichess.org/#puzzles)<br/>
and the project "chess-engine" (https://github.com/adam-mcdaniel/chess-engine/) which I used as a base for the GUI here.

## Usage:
Download the file "lichess_db_puzzle.csv" (from the lichess link above) to the puzzles directory.<br/>
Compilation is done with the usual "cargo build" in the project root.<br/><br/>

To play you just adjust the search to your needs, press "search" and a puzzle will be loaded, when you get it right the next puzzle will be immediatly displayed.<br/>
If the move is a promotion you need to select the piece to promote to in the botton of the screen before moving the pawn.<br/><br/>

The search is a bit slow but I think it's important to use the cvs directly so everyone can easily replace the file if needed.

## TODO:<br/>
- Externalize some strings (puzzle db filename, name of the image directory, limit of search results, etc.).<br/>
- Maybe allow the selection of multiple themes at once.<br/>
- Highlight the last move before the puzzle starts (so we can easily see if an en passant is allowed, for example)<br/>

## License:
Distributed under the MIT License.
