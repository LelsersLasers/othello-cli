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
    fn flip(&mut self) {
        *self = match self {
            Spot::Black => Spot::White,
            Spot::White => Spot::Black,
            Spot::Empty => Spot::Empty,
        };
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

fn print_game(board: [[Spot; 8]; 8], current_turn: Spot) {
    clear_screen();
    println!("   | 1 2 3 4 5 6 7 8 |");
    println!(" --+-----------------+");
    for (y, row) in board.iter().enumerate() {
        print!(" {} | ", y + 1);
        for spot in row.iter() {
            print!("{} ", spot.to_string());
        }
        println!("|");
    }
    println!("   +-----------------+");
    if current_turn != Spot::Empty {
        println!("Current turn: {}", current_turn.to_string());
    } else {
        println!("Game over!");
    }
}

fn invalid_move(board: [[Spot; 8]; 8], message: &'static str) -> [usize; 2] {
    println!("{}", message);
    get_input(board)
}

fn get_input(board: [[Spot; 8]; 8]) -> [usize; 2] {
    print!("Choose where to place piece (ex: '4 3'): ");

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
                board,
                "Incorrectly formatted input (entered more than 2 numbers?)",
            );
        } else if c.len() != 1 {
            return invalid_move(board, "Enter numbers only between 1 and 8");
        }
        let num = c.parse::<usize>();
        match num {
            Ok(n) => {
                if n > 8 {
                    return invalid_move(board, "Enter numbers only between 1 and 8");
                }
                pos[1 - count] = n - 1;
            }
            Err(_) => return invalid_move(board, "Enter numbers only between 1 and 8"),
        }
        count += 1;
    }
    if count != 2 {
        invalid_move(
            board,
            "Incorrectly formatted input (entered less than 2 numbers?)",
        )
    } else {
        pos
    }
}

fn main() {
    let mut board = create_board();
    let mut current_turn = Spot::Black;

    loop {
        print_game(board, current_turn);
        let input = get_input(board);
        board[input[0]][input[1]] = current_turn;

        current_turn.flip();
    }
}
