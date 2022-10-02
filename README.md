# offline-chess-puzzles
Tool to view and solve puzzles from the lichess puzzle database.

<img src="https://github.com/brianch/offline-chess-puzzles/blob/main/demo.gif" width="350"/>

It's a very simple tool for those who want to practice offline, it has filters by puzzle rating, theme and opening. It might also be helpful to teach the tactical motifs to students, since it's easy to select easy puzzles from a theme.

I need to thank lichess for creating the puzzle database (https://database.lichess.org/#puzzles)<br/>
and the project "chess-engine" (https://github.com/adam-mcdaniel/chess-engine/) which I used as a starting point for the GUI here.

## Usage:
First you need to download the file "lichess_db_puzzle.csv" (from the lichess link above) to the puzzles directory.<br/>
To play you simply adjust the search to your needs, press "search" and a puzzle will be loaded, when you get it right, the next puzzle will be immediatly displayed (you can disable this auto load in the settings).<br/><br/>
If the move is a promotion you need to select the piece to promote to in the bottom of the screen before moving the pawn.<br/><br/>
The search is a bit slow but I think it's important to use the cvs directly so users can easily replace the file if needed.<br/><br/>

## License:
- The code is distributed under the MIT License. See `LICENSE` for more information.<br/>
### Assets authors / licenses:
- The piece set "cburnett" is a work of Colin M.L. Burnett and used under the CC-BY-SA 3.0 unported license (more info on the license.txt file in that directory).
- The "california" piece set is a work of Jerry S. licensed under CC BY-NC-SA 4.0 (https://sites.google.com/view/jerrychess/home)
- The piece sets "Cardinal", "Dubrovny", "Gioco", "Icpieces", "Maestro", "Staunty" and "Tatiana" are work of "sadsnake1", licensed under CC BY-NC-SA 4.0. And obtained from the lila (lichess) repository.
