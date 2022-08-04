

#[derive(Copy, Clone)]
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
}

fn create_board() -> [[Spot; 8]; 8] {
    let mut board = [[Spot::Empty; 8]; 8];
    board[3][3] = Spot::White;
    board[3][4] = Spot::Black;
    board[4][3] = Spot::Black;
    board[4][4] = Spot::White;
    board
}


fn print_board(board: [[Spot; 8]; 8]) {
    println!("+-----------------+");
    for row in board.iter() {
        print!("| ");
        for spot in row.iter() {
            print!("{} ", spot.to_string());
        }
        println!("|");
    }
    println!("+-----------------+");
}


fn main() {
    println!("Hello, world!");
    let board = create_board();
    print_board(board);
}
