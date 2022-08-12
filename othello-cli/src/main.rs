use colored::Colorize;
use rand::seq::SliceRandom;
use std::io::prelude::*;

#[derive(Copy, Clone, PartialEq)]
enum Spot {
    Empty,
    Black(bool),
    White(bool),
}
impl Spot {
    fn to_string(self, colors: [Option<[u8; 3]>; 3], pieces: [char; 2]) -> colored::ColoredString {
        match self {
            Spot::Black(last_move) => match colors[0] {
                Some(color) => match last_move {
                    true => pieces[0]
                        .to_string()
                        .italic()
                        .bold()
                        .truecolor(color[0], color[1], color[2]),
                    false => pieces[0]
                        .to_string()
                        .truecolor(color[0], color[1], color[2]),
                },
                None => match last_move {
                    true => pieces[0].to_string().italic().bold().green(),
                    false => pieces[0].to_string().green(),
                },
            },
            Spot::White(last_move) => match colors[1] {
                Some(color) => match last_move {
                    true => pieces[1]
                        .to_string()
                        .italic()
                        .bold()
                        .truecolor(color[0], color[1], color[2]),
                    false => pieces[1]
                        .to_string()
                        .truecolor(color[0], color[1], color[2]),
                },
                None => match last_move {
                    true => pieces[1].to_string().italic().bold().red(),
                    false => pieces[1].to_string().red(),
                },
            },
            Spot::Empty => " ".clear(),
        }
    }
    fn get_flip(self) -> Spot {
        match self {
            Spot::Black(last_move) => Spot::White(last_move),
            Spot::White(last_move) => Spot::Black(last_move),
            Spot::Empty => Spot::Empty,
        }
    }
    fn get_true(self) -> Spot {
        match self {
            Spot::Black(_) => Spot::Black(true),
            Spot::White(_) => Spot::White(true),
            Spot::Empty => Spot::Empty,
        }
    }
    fn get_false(self) -> Spot {
        match self {
            Spot::Black(_) => Spot::Black(false),
            Spot::White(_) => Spot::White(false),
            Spot::Empty => Spot::Empty,
        }
    }
}

fn create_board() -> [[Spot; 8]; 8] {
    let mut board = [[Spot::Empty; 8]; 8];
    board[3][3] = Spot::White(false);
    board[3][4] = Spot::Black(false);
    board[4][3] = Spot::Black(false);
    board[4][4] = Spot::White(false);
    board
}

fn print_game(
    board: [[Spot; 8]; 8],
    valid_moves: &[[usize; 2]],
    current_turn: Spot,
    skip_turn: bool,
    colors: [Option<[u8; 3]>; 3],
    pieces: [char; 2],
) {
    print!("\x1B[2J\x1B[1;1H"); // clears screen

    println!("\n   | A B C D E F G H |");
    println!(" --+-----------------+");
    for y in 0..8 {
        print!(" {} | ", y + 1);
        // I think 0..8 is cleaner, but clippy wants this
        for (x, _item) in board.iter().enumerate() {
            if valid_moves.contains(&[x, y]) {
                match colors[2] {
                    Some(color) => {
                        print!("{} ", ".".truecolor(color[0], color[1], color[2]));
                    }
                    None => {
                        print!("{} ", ".".cyan());
                    }
                }
            } else {
                print!("{} ", board[x][y].to_string(colors, pieces));
            }
        }
        println!("|");
    }
    println!("   +-----------------+");

    let (black_total, white_total) = count_pieces(board);
    println!("{}'s: {}", pieces[0], black_total);
    println!("{}'s: {}", pieces[1], white_total);

    if current_turn != Spot::Empty {
        println!(
            "Current turn: {}",
            current_turn.get_false().to_string(colors, pieces)
        );
        if skip_turn {
            println!(
                "({}'s turn was skipped because they had no valid moves)",
                current_turn.get_flip().to_string(colors, pieces)
            );
        }
        print!("Valid moves: ");
        for (i, pos) in valid_moves.iter().enumerate() {
            let mut letters = "abcdefgh".chars();
            // TODO: Should be uppercase?
            // TODO: cleaner way than recreating a mutable iterator?
            print!(
                "{}{}",
                letters.nth(pos[0]).unwrap().to_uppercase(),
                pos[1] + 1
            );
            if i != valid_moves.len() - 1 {
                print!(", ");
            } else {
                println!();
            }
        }
    } else {
        println!("Game over!");
    }
}

fn end_game(board: [[Spot; 8]; 8], colors: [Option<[u8; 3]>; 3], pieces: [char; 2]) {
    print_game(board, &Vec::new(), Spot::Empty, false, colors, pieces);
    let (black_total, white_total) = count_pieces(board);
    if black_total == white_total {
        println!(
            "\n\nIt's a {}!\n",
            match colors[2] {
                Some(color) => {
                    "tie".truecolor(color[0], color[1], color[2])
                }
                None => {
                    "tie".cyan()
                }
            }
        );
    } else {
        println!(
            "\n\n{}'s wins!\n",
            if black_total > white_total {
                Spot::Black(false).to_string(colors, pieces)
            } else {
                Spot::White(false).to_string(colors, pieces)
            }
        );
    }
}

fn invalid_move(valid_moves: &Vec<[usize; 2]>, message: &'static str) -> [usize; 2] {
    println!("{}", message);
    get_input(valid_moves)
}

fn get_input(valid_moves: &Vec<[usize; 2]>) -> [usize; 2] {
    print!("Choose where to place piece: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input_str = input.trim().to_lowercase();

    let mut pos: [usize; 2] = [0; 2];

    if input_str.len() != 2 {
        return invalid_move(
            valid_moves,
            "Incorrectly formatted input (format input like: 'd3')",
        );
    }

    let valid_letters = "abcdefgh";
    let letter = input_str.chars().next().unwrap();
    if !valid_letters.contains(letter) {
        return invalid_move(
            valid_moves,
            "Incorrectly formatted input (enter only letters A-H)",
        );
    }
    pos[0] = valid_letters.find(letter).unwrap();

    let num = input_str.chars().nth(1).unwrap().to_digit(10);
    match num {
        Some(n) => match n {
            1..=8 => {
                pos[1] = n as usize - 1;
            }
            _ => {
                return invalid_move(
                    valid_moves,
                    "Incorrectly formatted input (enter numbers 1-8)",
                )
            }
        },
        None => {
            return invalid_move(
                valid_moves,
                "Incorrectly formatted input (enter numbers 1-8)",
            )
        }
    }

    if !valid_moves.contains(&pos) {
        invalid_move(valid_moves, "You cannot move there")
    } else {
        pos
    }
}

fn ai_input(valid_moves: &Vec<[usize; 2]>, wait_time: u64) -> [usize; 2] {
    // randomly moves
    std::thread::sleep(std::time::Duration::from_millis(wait_time));
    let choice = valid_moves.choose(&mut rand::thread_rng());
    match choice {
        Some(choice) => *choice,
        None => unreachable!(),
    }
}

fn ai_input_2(
    board: [[Spot; 8]; 8],
    current_turn: Spot,
    valid_moves: &Vec<[usize; 2]>,
    wait_time: u64,
) -> [usize; 2] {
    let mut best_moves: Vec<[usize; 2]> = Vec::new();
    let mut best_score = 0;
    for valid_move in valid_moves {
        let board_after_move = place_piece(board, *valid_move, current_turn);
        let (black_total, white_total) = count_pieces(board_after_move);
        let move_score = match current_turn {
            Spot::Black(_) => black_total,
            Spot::White(_) => white_total,
            _ => unreachable!(),
        };
        if move_score > best_score {
            best_moves = vec![*valid_move];
            best_score = move_score;
        } else if move_score == best_score {
            best_moves.push(*valid_move);
        }
    }

    std::thread::sleep(std::time::Duration::from_millis(wait_time));
    let choice = best_moves.choose(&mut rand::thread_rng());
    match choice {
        Some(choice) => *choice,
        None => unreachable!(),
    }
}

fn find_valid_moves(board: [[Spot; 8]; 8], current_turn: Spot) -> Vec<[usize; 2]> {
    let mut valid_moves: Vec<[usize; 2]> = vec![];
    for x in 0..8 {
        for y in 0..8 {
            if is_valid_move(board, [x, y], current_turn) {
                valid_moves.push([x, y]);
            }
        }
    }
    valid_moves
}

fn is_valid_move(board: [[Spot; 8]; 8], pos: [usize; 2], current_turn: Spot) -> bool {
    if board[pos[0]][pos[1]] != Spot::Empty {
        return false;
    }

    let dirs = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];
    for dir in dirs.iter() {
        let mut dist = 1;
        loop {
            let x = pos[0] as i32 + dir[0] * dist;
            let y = pos[1] as i32 + dir[1] * dist;
            if !(0..=7).contains(&x)
                || !(0..=7).contains(&y)
                || board[x as usize][y as usize] == Spot::Empty
                || (dist == 1
                    && board[x as usize][y as usize].get_true() != current_turn.get_flip())
            {
                break;
            } else if board[x as usize][y as usize].get_true() == current_turn {
                return true;
            }
            dist += 1;
        }
    }
    false
}

fn place_piece(mut board: [[Spot; 8]; 8], pos: [usize; 2], current_turn: Spot) -> [[Spot; 8]; 8] {
    // assumes valid move
    for x in 0..8 {
        for y in 0..8 {
            board[x][y] = board[x][y].get_false();
        }
    }
    board[pos[0]][pos[1]] = current_turn;
    let dirs = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];
    for dir in dirs.iter() {
        let mut dist = 1;
        let mut found = false;
        loop {
            let x = pos[0] as i32 + dir[0] * dist;
            let y = pos[1] as i32 + dir[1] * dist;
            if !(0..=7).contains(&x)
                || !(0..=7).contains(&y)
                || (dist == 1
                    && board[x as usize][y as usize].get_true() != current_turn.get_flip())
            {
                break;
            } else if board[x as usize][y as usize].get_true() == current_turn {
                found = true;
                break;
            }
            dist += 1;
        }
        if found {
            for i in 1..dist {
                let x = pos[0] as i32 + dir[0] * i;
                let y = pos[1] as i32 + dir[1] * i;
                board[x as usize][y as usize] = current_turn;
            }
        }
    }
    board
}

fn count_pieces(board: [[Spot; 8]; 8]) -> (u32, u32) {
    let mut black_total = 0;
    let mut white_total = 0;
    for row in board.iter() {
        for spot in row.iter() {
            match spot {
                Spot::Black(_last_move) => black_total += 1,
                Spot::White(_last_move) => white_total += 1,
                _ => (),
            }
        }
    }
    (black_total, white_total)
}

fn read_cli_options() -> ([bool; 2], [Option<[u8; 3]>; 3], [char; 2], u64) {
    let args: Vec<String> = std::env::args().map(|s| s.to_lowercase()).collect();
    let mut ais = [true; 2];

    let mut colors = [None; 3];
    let color_commands = [
        ["bc", "black-color"],
        ["wc", "white-color"],
        ["mc", "marked-color"],
    ];

    let mut pieces = ['X', 'O'];
    let piece_commands = [["bp", "black-piece"], ["wp", "white-piece"]];

    let mut ai_wait_time = 750;

    for arg in args.iter() {
        if arg == "h" || arg == "help" {
            println!("Usage: {} [options]", args[0]);
            println!("Options:");
            println!("  h, help\t\tShow this help message");
            println!("  b, black\t\tSet black to be controlled by the user");
            println!("  w, white\t\tSet white to be controlled by the user");
            println!("  bc, black-color\tSet the color of black to be a custom color");
            println!("\t\t\t  default: green");
            println!("\t\t\t  format: 'othello-cli black-color r g b' where r, g, and b are integers from 0-255");
            println!("  wc, white-color\tSet the color of white to be a custom color");
            println!("\t\t\t  default: red");
            println!("\t\t\t  format: 'othello-cli white-color r g b' where r, g, and b are integers from 0-255");
            println!("  mc, marked-color\tSet the color of the valid moves to be a custom color");
            println!("\t\t\t  default: cyan");
            println!("\t\t\t  format: 'othello-cli marked-color r g b' where r, g, and b are integers from 0-255");
            println!("  bp, black-piece\tSet the piece for black to be a custom character");
            println!("\t\t\t  default: X");
            println!("\t\t\t  format: 'othello-cli black-piece c' where c is a single character");
            println!("  wp, white-piece\tSet the piece for white to be a custom character");
            println!("\t\t\t  default: O");
            println!("\t\t\t  format: 'othello-cli white-piece c' where c is a single character");
            println!("  t, time\t\tSet the milliseconds the AI waits before making a move");
            println!("\t\t\t  default: 750 ms");
            println!("\t\t\t  format: 'othello-cli time ms' where ms is a positive integer");
            std::process::exit(0);
        } else if arg == "b" || arg == "black" {
            ais[0] = false;
        } else if arg == "w" || arg == "white" {
            ais[1] = false;
        } else if arg == "t" || arg == "time" {
            let ms_idx = args.iter().position(|s| s == arg).unwrap() + 1;
            if ms_idx >= args.len() || args[ms_idx].parse::<u64>().is_err() {
                println!("Invalid time");
                std::process::exit(1);
            }
            ai_wait_time = args[ms_idx].parse::<u64>().unwrap();
        } else {
            for i in 0..3 {
                if arg == color_commands[i][0] || arg == color_commands[i][1] {
                    let r_idx = args.iter().position(|s| s == arg).unwrap() + 1;
                    if r_idx >= args.len()
                        || r_idx + 1 >= args.len()
                        || r_idx + 2 >= args.len()
                        || args[r_idx].parse::<u8>().is_err()
                        || args[r_idx + 1].parse::<u8>().is_err()
                        || args[r_idx + 1].parse::<u8>().is_err()
                    {
                        println!("Invalid color format");
                        std::process::exit(1);
                    }
                    colors[i] = Some([
                        args[r_idx].parse::<u8>().unwrap(),
                        args[r_idx + 1].parse::<u8>().unwrap(),
                        args[r_idx + 2].parse::<u8>().unwrap(),
                    ]);
                }
            }
            for i in 0..2 {
                if arg == piece_commands[i][0] || arg == piece_commands[i][1] {
                    let c_idx = args.iter().position(|s| s == arg).unwrap() + 1;
                    if c_idx >= args.len() || args[c_idx].len() != 1 {
                        println!("Invalid piece");
                        std::process::exit(1);
                    }
                    pieces[i] = args[c_idx].chars().next().unwrap();
                }
            }
        }
    }

    (ais, colors, pieces, ai_wait_time)
}

fn turn(
    mut board: [[Spot; 8]; 8],
    mut current_turn: Spot,
    ais: [bool; 2],
    colors: [Option<[u8; 3]>; 3],
    pieces: [char; 2],
    ai_wait_time: u64,
) -> (bool, [[Spot; 8]; 8], Spot) {
    let mut valid_moves_current = find_valid_moves(board, current_turn);
    let valid_moves_opp = find_valid_moves(board, current_turn.get_flip());
    if valid_moves_current.is_empty() && valid_moves_opp.is_empty() {
        return (false, board, current_turn);
    }
    let skip_turn = valid_moves_current.is_empty();
    if skip_turn {
        current_turn = current_turn.get_flip();
        valid_moves_current = valid_moves_opp;
    }

    print_game(
        board,
        &valid_moves_current,
        current_turn,
        skip_turn,
        colors,
        pieces,
    );
    let input = if (current_turn == Spot::Black(true) && ais[0])
        || (current_turn == Spot::White(true) && ais[1])
    {
        if current_turn == Spot::Black(true) {
            ai_input_2(board, current_turn, &valid_moves_current, ai_wait_time)
        } else {
            ai_input(&valid_moves_current, ai_wait_time)
        }
    } else {
        get_input(&valid_moves_current)
    };
    board = place_piece(board, input, current_turn);

    current_turn = current_turn.get_flip();

    (true, board, current_turn)
}

fn main() {
    let mut board = create_board();
    let mut current_turn = Spot::Black(true);
    let mut game_on = true;

    let (ais, colors, pieces, ai_wait_time) = read_cli_options();

    while game_on {
        (game_on, board, current_turn) = turn(
            board,
            current_turn,
            ais,
            colors,
            pieces,
            ai_wait_time,
        );
    }
    end_game(board, colors, pieces);
}
