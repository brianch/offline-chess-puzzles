# offline-chess-puzzles
Tool to view and solve puzzles from the lichess puzzle database.

<img src="https://github.com/brianch/offline-chess-puzzles/blob/main/demo.gif" width="200"/>

It's a very simple tool for those who want to practice offline, it has filters by puzzle rating and theme. It might also be helpful to teach the tactical motifs to students, since it's easy to select easy puzzles from a theme, although the current lack of an analysis mode makes it a bit less practical for this use case.

I need to thank lichess for creating the puzzle database (https://database.lichess.org/#puzzles)<br/>
and the project "chess-engine" (https://github.com/adam-mcdaniel/chess-engine/) which I used as a starting point for the GUI here.

## Usage:
If you will compile the sources yourself, you also need to download the file "lichess_db_puzzle.csv" (from the lichess link above) to the puzzles directory.<br/>
Compilation is done with the usual "cargo build" in the project root.<br/><br/>
To play you just adjust the search to your needs, press "search" and a puzzle will be loaded, when you get it right the next puzzle will be immediatly displayed.<br/>
If the move is a promotion you need to select the piece to promote to in the botton of the screen before moving the pawn.<br/><br/>
The search is a bit slow but I think it's important to use the cvs directly so users can easily replace the file if needed.

## License:
- The code is distributed under the MIT License. See `LICENSE` for more information.<br/>
- The images in the "cburnett" directory are from Colin M.L. Burnett and used under the CC-BY-SA 3.0 unported license (there's a link to the original files in the file license.txt in that directory).
