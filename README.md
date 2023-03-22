# offline-chess-puzzles
[![Build](https://github.com/brianch/offline-chess-puzzles/actions/workflows/build.yml/badge.svg)](https://github.com/brianch/offline-chess-puzzles/actions/workflows/build.yml) <a href="https://ko-fi.com/O4O7JPA8E"><img src="https://ko-fi.com/img/githubbutton_sm.svg" alt="support me on ko-fi" height="20"/></a>

Very simple tool to view and solve puzzles from the lichess puzzle database.

<img src="https://user-images.githubusercontent.com/5335499/223898478-8b108594-67aa-4ad0-abd4-eb017db9bb9c.gif" width="400"/>

<br>I need to thank lichess for creating the [puzzle database](https://database.lichess.org/#puzzles) and the project [chess-engine](https://github.com/adam-mcdaniel/chess-engine/) which I used as a starting point for the GUI here.

## Usage:
Download the app in the [releases page](https://github.com/brianch/offline-chess-puzzles/releases) here.

You'll also need to download the file "lichess_db_puzzle.csv" (from the lichess link above) to the "puzzles" directory of the app.

To play you simply adjust the search to your needs, press "search" and a puzzle will be loaded, when you get it right, the next one will be immediatly displayed (you can disable this auto load in the settings).

If the move is a promotion you need to select the piece to promote to (in the search tab) before moving the pawn.

The search is a bit slow (especially when searching by opening, because it often needs to traverse the whole database) but I think it's important to use the cvs directly so users can easily replace the file if needed.

## Possible use cases:
- Practice offline, it has filters by puzzle rating, theme and opening.
- Teach the tactical motifs to students, since it's simple to select easy puzzles from a theme (it lack arrows, but there's an analysis function)
- Setting a very small search limit might be useful for those who want to practice by repetition (you'll get the same puzzles each time, in random order). But there's no build-in functionality specific for this yet.

Are you using this app? I'd be very interested in knowing what's your use case and if there's any other feature that would be useful. Feel free to start a conversation in [discussions](https://github.com/brianch/offline-chess-puzzles/discussions) (for general feedback/ideas) or to create an [issue](https://github.com/brianch/offline-chess-puzzles/issues) (to report bugs or specific feature requests).

I can't promise anything though, since this is just a hobby project (and the goal is to keep the app simple).

## Features
- Almost all the filters we have in Lichess, plus rating range (the exception is that we only have filter by opening family, not by each variation)
- Flip the board to solve from the opponent's perspective (to practice seeing what is being threated against us)
- A few piece themes and a bunch of board themes
- Analysis board (with basic engine support)
- Hint (see which piece to move)
- Settings are remembered and loaded when you open the app again

## License:
- The code is distributed under the MIT License. See `LICENSE` for more information.<br/>
### Assets authors / licenses:
- The piece set "cburnett" is a work of Colin M.L. Burnett and used under the CC-BY-SA 3.0 unported license (more info on the license.txt file in that directory).
- The "california" piece set is a work of Jerry S. licensed under CC BY-NC-SA 4.0 (https://sites.google.com/view/jerrychess/home)
- The piece sets "Cardinal", "Dubrovny", "Gioco", "Icpieces", "Maestro", "Staunty" and "Tatiana" are work of "sadsnake1", licensed under CC BY-NC-SA 4.0. And obtained from the lila (lichess) repository.
