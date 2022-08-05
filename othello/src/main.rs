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

fn print_game(board: [[Spot; 8]; 8], valid_moves: [[bool; 8]; 8], valid_moves_vec: Vec<[usize; 2]>, current_turn: Spot) {
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
    println!("X's: {} ", black_total);
    println!("O's: {}", white_total);

    if current_turn != Spot::Empty {
        println!("Current turn: {}", current_turn.to_string());
        // if valid_moves.len() == 0 {
        //     println!("No valid moves, skipped {}'s turn", current_turn.get_flip().to_string());
        // }
        print!("Valid moves: ");
        for (i, pos) in valid_moves_vec.iter().enumerate() {
            print!("'{} {}'", pos[1] + 1, pos[0] + 1);
            if i != valid_moves_vec.len() - 1 {
                print!(", ");
            } else {
                println!("");
            }
        }
    } else {
        println!("Game over!");
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

fn find_valid_moves(board: [[Spot; 8]; 8], current_turn: Spot) -> [[bool; 8]; 8] {
    let mut valid_moves = [[false; 8]; 8];
    for y in 0..8 {
        for x in 0..8 {
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

fn main() {
    let mut board = create_board();
    // board[1][3] = Spot::White;
    let mut current_turn = Spot::Black;
    let mut no_valid_modes = false;

    loop {
        let valid_moves_current = find_valid_moves(board, current_turn);
        let valid_moves_current_vec = valid_moves_to_vec(valid_moves_current);
        let valid_moves_opp = find_valid_moves(board, current_turn.get_flip());
        let valid_moves_opp_vec = valid_moves_to_vec(valid_moves_opp);

        if valid_moves_current_vec.len() == 0 && valid_moves_opp_vec.len() == 0 {
            break;
        }

        print_game(board, valid_moves_current, valid_moves_current_vec, current_turn);
        let input = get_input(valid_moves_current);
        place_piece(&mut board, input, current_turn);

        current_turn = current_turn.get_flip();
    }
}
