use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
const BOARD_SIZE: usize = 3;
type Board = [[char; BOARD_SIZE]; BOARD_SIZE];

fn initialize_board() -> Board {
    [[' '; BOARD_SIZE]; BOARD_SIZE]
}


fn print_board(board: &Board) {
    for row in board {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

fn get_player_move(current_player: char, board: &Board)->(usize, usize) {
    loop{
        println!("Player {}, enter your move (row and column): ", current_player);
        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).expect("failed to read input");
        if bytes_read == 0 {
            println!("Input closed. Exiting.");
            std::process::exit(0);
        }
        println!("You entered: {}", input);

        let coordinates: Vec<usize> = input
            .split_whitespace()
            .flat_map(str::parse::<usize>)
            .collect();
        if coordinates.len() == 2 {
            let (row, col) = (coordinates[0], coordinates[1]);
            if row < BOARD_SIZE && col < BOARD_SIZE && board[row][col] == ' ' {
                return (row, col);
            } 

        }
        println!("Invalid move. Please try again.");
    }
    
}

fn check_winner(player: char, board: &Board) -> bool {
    // row
    for row in 0..BOARD_SIZE {
        if board[row][0] == player && board[row][1] == player && board[row][2] == player {
            return true;
        }
    }
    // column
    for col in 0..BOARD_SIZE {
        if board[0][col] == player && board[1][col] == player && board[2][col] == player {
            return true;
        }
    }
    // diagonal
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }
    false
}

fn check_draw(board: &Board) -> bool {
    for row in board {
        for cell in row {
            if *cell == ' ' {
                return false;
            }
        }
    }
   return true;
}


fn  play_game(){
    let mut board = initialize_board();
    let mut current_player = PLAYER_X;
    loop {
        println!("Current Board ");
        print_board(&board);

        let (row, col) = get_player_move(current_player, &board);
        board[row][col] = current_player;
        if check_winner(current_player, &board){
            println!("Player {} wins!", current_player);

            print_board(&board);
            break;
        }
        if check_draw(&board){
            println!("It's a draw!");
            print_board(&board);
            break;
        }
        current_player = if current_player == PLAYER_X {
            PLAYER_O
        } else {
            PLAYER_X
        };
}

}
fn main(){
    print!("welcome to tic tac toe game");
    play_game();
}