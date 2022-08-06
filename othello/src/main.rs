use rand::seq::SliceRandom;
use std::io::prelude::*;

#[derive(Copy, Clone, PartialEq)]
enum Spot {
    Empty,
    Black,
    White,
}
impl Spot {
    fn to_string(self) -> &'static str {
        match self {
            Spot::Black => "X",
            Spot::White => "O",
            Spot::Empty => " ",
        }
    }
    fn get_flip(self) -> Spot {
        match self {
            Spot::Black => Spot::White,
            Spot::White => Spot::Black,
            Spot::Empty => Spot::Empty,
        }
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn create_board() -> [[Spot; 8]; 8] {
    let mut board = [[Spot::Empty; 8]; 8];
    board[3][3] = Spot::White;
    board[3][4] = Spot::Black;
    board[4][3] = Spot::Black;
    board[4][4] = Spot::White;
    board
}

fn print_game(
    board: [[Spot; 8]; 8],
    valid_moves: [[bool; 8]; 8],
    valid_moves_vec: Vec<[usize; 2]>,
    current_turn: Spot,
    skip_turn: bool,
) {
    clear_screen();

    println!("\n   | 1 2 3 4 5 6 7 8 |");
    println!(" --+-----------------+");
    for y in 0..8 {
        print!(" {} | ", y + 1);
        for x in 0..8 {
            if valid_moves[x][y] {
                print!(". ");
            } else {
                print!("{} ", board[x][y].to_string());
            }
        }
        println!("|");
    }
    println!("   +-----------------+");

    let (black_total, white_total) = count_pieces(board);
    println!("X's: {}", black_total);
    println!("O's: {}", white_total);

    if current_turn != Spot::Empty {
        println!("Current turn: {}", current_turn.to_string());
        if skip_turn {
            println!(
                "({}'s turn was skipped because they had no valid moves)",
                current_turn.get_flip().to_string()
            );
        }
        print!("Valid moves: ");
        for (i, pos) in valid_moves_vec.iter().enumerate() {
            print!("'{} {}'", pos[0] + 1, pos[1] + 1);
            if i != valid_moves_vec.len() - 1 {
                print!(", ");
            } else {
                println!();
            }
        }
    } else {
        println!("Game over!");
    }
}

fn end_game(board: [[Spot; 8]; 8]) {
    print_game(board, [[false; 8]; 8], vec![], Spot::Empty, false);
    let (black_total, white_total) = count_pieces(board);
    match black_total {
        x if x > white_total => println!("\n\nX wins!\n"),
        x if x < white_total => println!("\n\nO wins!\n"),
        _ => println!("\n\nIt's a tie!\n"),
    }
}

fn invalid_move(valid_moves: [[bool; 8]; 8], message: &'static str) -> [usize; 2] {
    println!("{}", message);
    get_input(valid_moves)
}

fn get_input(valid_moves: [[bool; 8]; 8]) -> [usize; 2] {
    print!("Choose where to place piece: ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let string_parts = input.trim().split(' ');

    let mut count = 0;
    let mut pos: [usize; 2] = [0; 2];
    for c in string_parts {
        if count >= 2 {
            return invalid_move(
                valid_moves,
                "Incorrectly formatted input (entered more than 2 numbers?)",
            );
        } else if c.len() != 1 {
            return invalid_move(valid_moves, "Enter numbers only between 1 and 8");
        }
        let num = c.parse::<usize>();
        match num {
            Ok(n) => {
                if n > 8 {
                    return invalid_move(valid_moves, "Enter numbers only between 1 and 8");
                }
                pos[count] = n - 1;
            }
            Err(_) => return invalid_move(valid_moves, "Enter numbers only between 1 and 8"),
        }
        count += 1;
    }
    if count != 2 {
        invalid_move(
            valid_moves,
            "Incorrectly formatted input (entered less than 2 numbers?)",
        )
    } else if !valid_moves[pos[0]][pos[1]] {
        invalid_move(valid_moves, "You cannot move there")
    } else {
        pos
    }
}

fn ai_input(valid_moves: [[bool; 8]; 8], wait_time: u64) -> [usize; 2] {
    // randomly moves
    std::thread::sleep(std::time::Duration::from_millis(wait_time)); // wait 0.6 sec
    let valid_moves_vec = valid_moves_to_vec(valid_moves);
    let choice = valid_moves_vec.choose(&mut rand::thread_rng());
    match choice {
        Some(choice) => *choice,
        None => unreachable!(),
    }
}

fn find_valid_moves(board: [[Spot; 8]; 8], current_turn: Spot) -> [[bool; 8]; 8] {
    let mut valid_moves = [[false; 8]; 8];
    for x in 0..8 {
        for y in 0..8 {
            if valid_move(board, [x, y], current_turn) {
                valid_moves[x][y] = true;
            }
        }
    }
    valid_moves
}

fn valid_moves_to_vec(valid_moves: [[bool; 8]; 8]) -> Vec<[usize; 2]> {
    let mut valid_moves_vec: Vec<[usize; 2]> = vec![];
    for y in 0..8 {
        for x in 0..8 {
            if valid_moves[x][y] {
                valid_moves_vec.push([x, y]);
            }
        }
    }
    valid_moves_vec
}

fn valid_move(board: [[Spot; 8]; 8], pos: [usize; 2], current_turn: Spot) -> bool {
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
                || (dist == 1 && board[x as usize][y as usize] != current_turn.get_flip())
            {
                break;
            } else if board[x as usize][y as usize] == current_turn {
                return true;
            }
            dist += 1;
        }
    }
    false
}

fn place_piece(board: &mut [[Spot; 8]; 8], pos: [usize; 2], current_turn: Spot) {
    // assumes valid move
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
                || (dist == 1 && board[x as usize][y as usize] != current_turn.get_flip())
            {
                break;
            } else if board[x as usize][y as usize] == current_turn {
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
}

fn count_pieces(board: [[Spot; 8]; 8]) -> (u32, u32) {
    let mut black_total = 0;
    let mut white_total = 0;
    for row in board.iter() {
        for spot in row.iter() {
            match spot {
                Spot::Black => black_total += 1,
                Spot::White => white_total += 1,
                _ => (),
            }
        }
    }
    (black_total, white_total)
}

fn read_cli_options() -> (bool, bool, u64) {
    let args: Vec<String> = std::env::args().map(|s| s.to_lowercase()).collect();
    let mut black_is_ai = true;
    let mut white_is_ai = true;
    let mut ai_wait_time = 750;

    for arg in args.iter() {
        if arg == "h" || arg == "help" {
            println!("Usage: {} [options]", args[0]);
            println!("Options:");
            println!("  h, help\t\tShow this help message");
            println!("  b, black\t\tSet black to be controlled by the user");
            println!("  w, white\t\tSet white to be controlled by the user");
            println!("  t, time\t\tSet the time (in milliseconds) the AI waits before making a move (default: 750 ms)");
            std::process::exit(0);
        } else if arg == "b" || arg == "black" {
            black_is_ai = false;
        } else if arg == "w" || arg == "white" {
            white_is_ai = false;
        } else if arg == "t" || arg == "time" {
            let time = args[args.iter().position(|s| s == arg).unwrap() + 1]
                .parse::<u64>()
                .unwrap();
            ai_wait_time = time;
        }
    }

    (black_is_ai, white_is_ai, ai_wait_time)
}

fn main() {
    let mut board = create_board();
    let mut current_turn = Spot::Black;

    let (black_is_ai, white_is_ai, ai_wait_time) = read_cli_options();

    loop {
        let mut valid_moves_current = find_valid_moves(board, current_turn);
        let mut valid_moves_current_vec = valid_moves_to_vec(valid_moves_current);
        let valid_moves_opp = find_valid_moves(board, current_turn.get_flip());
        let valid_moves_opp_vec = valid_moves_to_vec(valid_moves_opp);
        if valid_moves_current_vec.is_empty() && valid_moves_opp_vec.is_empty() {
            break;
        }
        let skip_turn = valid_moves_current_vec.is_empty();
        if skip_turn {
            current_turn = current_turn.get_flip();
            valid_moves_current = valid_moves_opp;
            valid_moves_current_vec = valid_moves_opp_vec;
        }

        print_game(
            board,
            valid_moves_current,
            valid_moves_current_vec,
            current_turn,
            skip_turn,
        );
        let input = if (current_turn == Spot::Black && black_is_ai)
            || (current_turn == Spot::White && white_is_ai)
        {
            ai_input(valid_moves_current, ai_wait_time)
        } else {
            get_input(valid_moves_current)
        };
        place_piece(&mut board, input, current_turn);

        current_turn = current_turn.get_flip();
    }
    end_game(board);
}
