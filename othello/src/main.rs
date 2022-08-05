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
    println!("\n   | 1 2 3 4 5 6 7 8 |");
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
                pos[1 - count] = n - 1; // 1 - count because we want to reverse the order
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
        let mut dist = 2;
        let mut found = false;
        loop {
            let x = pos[0] as i32 + dir[0] * dist;
            let y = pos[1] as i32 + dir[1] * dist;
            if !(0..=7).contains(&x) || !(0..=7).contains(&y) || board[x as usize][y as usize] == Spot::Empty {
                break;
            }
            if board[x as usize][y as usize] == current_turn {
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

fn main() {
    let mut board = create_board();
    let mut current_turn = Spot::Black;

    loop {
        print_game(board, current_turn);
        let input = get_input(board);
        place_piece(&mut board, input, current_turn);

        current_turn.flip();
    }
}
