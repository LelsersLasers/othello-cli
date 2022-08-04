

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
    fn next_turn(&mut self) {
        match self {
            Spot::Black => *self = Spot::White,
            Spot::White => *self = Spot::Black,
            _ => unreachable!(),
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


fn print_game(board: [[Spot; 8]; 8], current_turn: Spot) {
    clear_screen();
    println!("+-----------------+");
    for row in board.iter() {
        print!("| ");
        for spot in row.iter() {
            print!("{} ", spot.to_string());
        }
        println!("|");
    }
    println!("+-----------------+");
    if current_turn != Spot::Empty {
        println!("Current turn: {}", current_turn.to_string());
    } else {
        println!("Game over!");
    }
}


fn main() {
    let mut board = create_board();
    let mut current_turn = Spot::Black;

    loop {
        print_game(board, current_turn);

        current_turn.next_turn();
    }
}
