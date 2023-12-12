use std::collections::VecDeque;
use std::str::FromStr;
use lopdf::dictionary;
use lopdf::{Document, Object, Stream};
use lopdf::content::{Content, Operation};
use chess::{Board, ChessMove, Color, Piece, Square};

use crate::{config, PuzzleTab};

pub fn to_pdf(puzzles: &Vec<config::Puzzle>, number_of_pages: i32) {
    let font_data = std::fs::read("font/Alpha.ttf").unwrap();
    // Load the font data from a file

    // Create a stream object for the font data
    let font_stream = Stream::new(dictionary! {}, font_data.clone());

    // Create a document object and add the font and font descriptor to it
    let mut doc = Document::with_version("1.7");
    // Create a font descriptor dictionary object
    let font_stream_id = doc.add_object(font_stream);
    let font_descriptor_dict = dictionary! {
        "Type" => "TrueType",
        "FontName" => "Chess-Maya",
        "FontFile2" => font_stream_id,
        "Flags" => 1,
    };
    let font_descriptor_id = doc.add_object(font_descriptor_dict);

    // Create a font dictionary object
    let font_dict = dictionary! {
        "Type" => "Font",
        "Subtype" => "TrueType",
        "BaseFont" => "Chess-Maya",
        "FirstChar" => 0,
        "LastChar" => 255,
        "Widths" => vec![1000.into();256],
        "FontDescriptor" => font_descriptor_id,
    };

    let font_id = doc.add_object(font_dict);
    // pages is the root node of the page tree
    let pages_id = doc.new_object_id();

    let regular_font_id = doc.add_object(dictionary! {
        // type of dictionary
        "Type" => "Font",
        // type of font, type1 is simple postscript font
        "Subtype" => "TrueType",
        // basefont is postscript name of font for type1 font.
        // See PDF reference document for more details
        "BaseFont" => "Arial",
    });
    // font dictionaries need to be added into resource dictionaries
    // in order to be used.
    // Resource dictionaries can contain more than just fonts,
    // but normally just contains fonts
    // Only one resource dictionary is allowed per page tree root
    let resources_id = doc.add_object(dictionary! {
        // fonts are actually triplely nested dictionaries. Fun!
        "Font" => dictionary! {
            "Chess-Maya" => font_id,
            "Regular" => regular_font_id,
        },
    });

    let num_of_puzzles_to_print;
    let num_of_pages;
    if (6 * number_of_pages) as usize > puzzles.len() {
        num_of_puzzles_to_print = puzzles.len();
        num_of_pages = puzzles.len() % 6;
    } else {
        num_of_puzzles_to_print = (6 * number_of_pages) as usize;
        num_of_pages = number_of_pages as usize;
    };

    //let number_of_pages: i64 = 100;//(puzzles.len() / 6).try_into().unwrap();
    let mut page_ids = vec![];
    let mut puzzle_index = 0;
    for _ in 0..num_of_pages.into() {
        let mut ops: Vec<Operation> = vec![];
        let mut pos_x = 750;
        let mut pos_y = 75;
        for i in 0..6 {
            if puzzle_index == puzzles.len() { break };
            ops.append(&mut gen_diagram_operations(puzzle_index + 1, &puzzles[puzzle_index], pos_x, pos_y));
            if i % 2 == 0 {
                pos_y = 325;
            } else {
                pos_y = 75;
                pos_x -= 250;
            };
            puzzle_index += 1;
        }

        // Content is a wrapper struct around an operations struct that contains a vector of operations
        // The operations struct contains a vector of operations that match up with a particular PDF
        // operator and operands.
        // Reference the PDF reference for more details on these operators and operands.
        // Note, the operators and operands are specified in a reverse order than they
        // actually appear in the PDF file itself.
        let content = Content {
            operations: ops,
        };

        // Streams are a dictionary followed by a sequence of bytes. What that sequence of bytes
        // represents depends on context
        // The stream dictionary is set internally to lopdf and normally doesn't
        // need to be manually nanipulated. It contains keys such as
        // Length, Filter, DecodeParams, etc
        //
        // content is a stream of encoded content data.
        let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));

        // Page is a dictionary that represents one page of a PDF file.
        // It has a type, parent and contents
        //let page_id = doc.add_object(dictionary! {
        page_ids.push(doc.add_object(dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
        }).into());
    }
    
    let mut ops: Vec<Operation> = vec![];
    let mut pos_x = 800;
    let pos_y = 75;
    let mut num_pages_of_solution = 1;
    for puzzle_number in 0..num_of_puzzles_to_print {
        // need to start by making the 1st move in the list, because it's only then that
        // the puzzle starts.
        let mut board = Board::from_str(&puzzles[puzzle_number].fen).unwrap();
        let mut puzzle_moves: VecDeque<&str> = puzzles[puzzle_number].moves.split_whitespace().collect();
        let movement = ChessMove::new(
            Square::from_str(&String::from(&puzzle_moves[0][..2])).unwrap(),
            Square::from_str(&String::from(&puzzle_moves[0][2..4])).unwrap(), PuzzleTab::check_promotion(puzzle_moves[0]));
        board = board.make_move_new(movement);

        let mut solution = (puzzle_number + 1).to_string() + ") ";
        // Remove the opponent's first move, it's not part of the solution.
        puzzle_moves.pop_front();

        let mut half_move_number = 1;
        let mut move_label = 1;
        if board.side_to_move() == Color::Black {
            solution.push_str(" 1. ... ");
            half_move_number = 2;
            move_label = 2;
        }
        for chess_move in puzzle_moves {
            if half_move_number % 2 == 0 {
                solution.push_str(" ");
                solution.push_str(&config::coord_to_san(&board, String::from(chess_move)).unwrap());
            } else {
                solution.push_str(" ");
                solution.push_str(&move_label.to_string());
                solution.push_str(". ");
                solution.push_str(&config::coord_to_san(&board, String::from(chess_move)).unwrap());
                move_label = move_label + 1;
            }
            half_move_number = half_move_number + 1;
            // Apply move, so we have the updated board to generate the SAN for the next move.
            let movement = ChessMove::new(
                Square::from_str(&String::from(&chess_move[..2])).unwrap(),
                Square::from_str(&String::from(&chess_move[2..4])).unwrap(), PuzzleTab::check_promotion(chess_move));
            board = board.make_move_new(movement);
        }
        ops.append(&mut vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["Regular".into(), 12.into()]),
                Operation::new("rg", vec![0.into(),0.into(),0.into()]),
                Operation::new("Td", vec![pos_y.into(), pos_x.into()]),
                Operation::new("Tj", vec![Object::string_literal(solution)]),
                Operation::new("ET", vec![]),
        ]);
        pos_x = pos_x - 18;

        // We need a page break
        if pos_x < 18 {
            pos_x = 800;
            num_pages_of_solution = num_pages_of_solution + 1;

            let content = Content {
                operations: ops,
            };
        
            let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
            page_ids.push(doc.add_object(dictionary! {
                "Type" => "Page",
                "Parent" => pages_id,
                "Contents" => content_id,
            }).into());
            ops = vec![];
        }
    }
    let content = Content {
        operations: ops,
    };

    let content_id = doc.add_object(Stream::new(dictionary! {}, content.encode().unwrap()));
    page_ids.push(doc.add_object(dictionary! {
        "Type" => "Page",
        "Parent" => pages_id,
        "Contents" => content_id,
    }).into());

    // Again, pages is the root of the page tree. The ID was already created
    // at the top of the page, since we needed it to assign to the parent element of the page
    // dictionary
    //
    // This is just the basic requirements for a page tree root object. There are also many
    // additional entries that can be added to the dictionary if needed. Some of these can also be
    // defined on the page dictionary itself, and not inherited from the page tree root.
    let pages = dictionary! {
        // Type of dictionary
        "Type" => "Pages",
        // Page count
        "Count" => Object::Integer(page_ids.len() as i64),
        // Vector of page IDs in document. Normally would contain more than one ID and be produced
        // using a loop of some kind
        "Kids" => page_ids,
        // ID of resources dictionary, defined earlier
        "Resources" => resources_id,
        // a rectangle that defines the boundaries of the physical or digital media. This is the
        // "Page Size"
        "MediaBox" => vec![0.into(), 0.into(), 600.into(), 850.into()],
    };

    // using insert() here, instead of add_object() since the id is already known.
    doc.objects.insert(pages_id, Object::Dictionary(pages));

    // Creating document catalog.
    // There are many more entries allowed in the catalog dictionary.
    let catalog_id = doc.add_object(dictionary! {
        "Type" => "Catalog",
        "Pages" => pages_id,
    });

    // Root key in trailer is set here to ID of document catalog,
    // remainder of trailer is set during doc.save().
    doc.trailer.set("Root", catalog_id);
    doc.compress();

    // Store file in current working directory.
    doc.save("example.pdf").unwrap();
}

fn gen_diagram_operations(index: usize, puzzle: &config::Puzzle, start_x:i32, start_y:i32) -> Vec<Operation> {
    let mut board = Board::from_str(&puzzle.fen).unwrap();
    let puzzle_moves: Vec<&str> = puzzle.moves.split_whitespace().collect();
    let movement = ChessMove::new(
        Square::from_str(&String::from(&puzzle_moves[0][..2])).unwrap(),
        Square::from_str(&String::from(&puzzle_moves[0][2..4])).unwrap(), PuzzleTab::check_promotion(puzzle_moves[0]));

    let last_move = if board.side_to_move() == Color::White {
        index.to_string() + ") Black to move. Last move: " + &config::coord_to_san(&board, String::from(&puzzle_moves[0][0..4])).unwrap()
    } else {
        index.to_string() + ") White to move. Last move: ... " + &config::coord_to_san(&board, String::from(&puzzle_moves[0][0..4])).unwrap()
    };
    board = board.make_move_new(movement);

    let mut ops = vec![
            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["Regular".into(), 10.into()]),
            Operation::new("rg", vec![0.into(),0.into(),0.into()]),
            Operation::new("Td", vec![start_y.into(), (start_x + 30).into()]),
            Operation::new("Tj", vec![Object::string_literal(last_move)]),
            Operation::new("ET", vec![]),

            Operation::new("BT", vec![]),
            Operation::new("Tf", vec!["Chess-Maya".into(), 25.into()]),
            Operation::new("rg", vec![0.into(),0.into(),0.into()]),
            Operation::new("Td", vec![start_y.into(), start_x.into()]),
            ];

    let ranks = (0..8).rev().collect::<Vec<i32>>();
    let files = (0..8).collect::<Vec<i32>>();
    for rank in ranks {
        let mut rank_string = String::new();
        for file in &files {
            let mut new_piece;
            let light_square = (rank + file) % 2 != 0;
            let square = chess::Square::make_square(chess::Rank::from_index(rank as usize),chess::File::from_index(*file as usize));
            let (piece, color) =
                (board.piece_on(square),
                board.color_on(square));

            if let Some(piece) = piece {
                if color.unwrap() == Color::White {
                    match piece {
                        Piece::Pawn => new_piece = 'P',
                        Piece::Rook => new_piece = 'R',
                        Piece::Knight => new_piece = 'H',
                        Piece::Bishop => new_piece = 'B',
                        Piece::Queen => new_piece = 'Q',
                        Piece::King => new_piece = 'K',
                    }
                    if light_square {
                        new_piece = new_piece.to_lowercase().collect::<Vec<_>>()[0];
                    }
                } else {
                    match piece {
                        Piece::Rook => new_piece = 'T',
                        Piece::Knight => new_piece = 'J',
                        Piece::Bishop => new_piece = 'N',
                        Piece::Queen => new_piece = 'W',
                        Piece::King => new_piece = 'L',
                        Piece::Pawn => new_piece = 'O',
                    }
                    if light_square {
                        new_piece = new_piece.to_lowercase().collect::<Vec<_>>()[0];
                    }
                }
            } else {
                if light_square {
                    new_piece = '0';
                } else {
                    new_piece = '+';
                }
            }
            rank_string.push(new_piece);
        }
        ops.push(Operation::new("Tj", vec![Object::string_literal(rank_string)]));
        ops.push(Operation::new("Td", vec![0.into(), Object::Integer(-25)]));
    }
    ops.push(Operation::new("ET", vec![]));
    ops
}

