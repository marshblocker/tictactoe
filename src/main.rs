// Tic-tac-toe game.

use std::io;
use std::collections::HashMap;

const ROWS: usize = 3;
const COLS: usize = 3;

#[derive(PartialEq)]
enum InputStatus {
    Success,
    NotTwoDigits,
    RowNotBaseTen,
    ColNotBaseTen,
    InvalidRowVal,
    InvalidColVal,
    GridOccupied,
}

// This determines the state of each grid and is also used
// as output of checking who won the game (X - first player won,
// O - second player won, EMPTY - no one won yet).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum State {
    X,
    O,
    EMPTY,
}

type Board = [[State; COLS]; ROWS];

fn main() {
    let mut board: Board = [[State::EMPTY; COLS]; ROWS];
    let mapping = HashMap::from([
        (State::X, 'X'),
        (State::O, 'O'),
        (State::EMPTY, ' '),
    ]);

    // Increments when a valid move is made. If its value becomes 9,
    // then the board is full and the game is draw.
    let mut valid_moves = 0;

    // First turn goes to player X.
    let mut turn = State::X;

    clear_screen();
    print_board(&board, &mapping);

    let mut winner = State::EMPTY;
    while winner == State::EMPTY && valid_moves != 9 {
        let status = move_player(&mut board, turn);

        if status != InputStatus::Success {
            clear_screen();
            print_board(&board, &mapping);
            print_input_status(status);
            continue;
        }

        valid_moves += 1;

        clear_screen();
        print_board(&board, &mapping);

        match check_winner(&board) {
            State::EMPTY => (),
            player => winner = player,
        }

        // Swap turn.
        turn = match turn {
            State::X     => State::O,
            State::O     => State::X,
            State::EMPTY => panic!("Invalid turn value!"),
        };
    }

    match winner {
        State::X     => println!("\nPlayer X wins!"),
        State::O     => println!("\nPlayer O wins!"),
        State::EMPTY => println!("\nDraw!"), // When valid_moves == 9.
    }
}

// Based on the turn value, either Player X or Player O will move.
// If the given move is valid, update the board and return true to
// the calling function (which signals the given input is valid).
// If not, then return false so the current player can choose his/her
// move again.
fn move_player(board: &mut Board, turn: State) -> InputStatus {
    println!("\n");
    match turn {
        State::X => println!("Player X turn."),
        State::O => println!("Player O turn."),
        State::EMPTY => panic!("Invalid turn value!"),
    }

    println!("Input your move in 'rowcol' format (e.g. '11' or '33'):");

    let mut move_str = String::new();

    io::stdin()
        .read_line(&mut move_str)
        .expect("Error recieving input!");

    let move_str: &str = move_str.trim();

    if move_str.len() != 2 {
        return InputStatus::NotTwoDigits;
    }

    let mut chars = move_str.chars();

    // Extract row value from the user input.
    let r = chars.next()
            .expect("Invalid character given to row")
            .to_digit(10);

    let r = match r {
        Some(digit) => digit,
        None        => return InputStatus::RowNotBaseTen,
    };

    let r: usize = match r {
        1|2|3 => (r - 1) as usize,
        _     => {
            return InputStatus::InvalidRowVal;
        }
    };

    // Extract column value from the user input.
    let c = chars.next()
            .expect("Invalid character given to column.")
            .to_digit(10);

    let c = match c {
        Some(digit) => digit,
        None        => return InputStatus::ColNotBaseTen,
    };

    let c: usize = match c {
        1|2|3 => (c - 1) as usize,
        _     => {
            return InputStatus::InvalidColVal;
        }
    };

    if board[r][c] != State::EMPTY {
        return InputStatus::GridOccupied;
    }

    // Update board based on the player move.
    match turn {
        State::X     => board[r][c] = State::X,
        State::O     => board[r][c] = State::O,
        State::EMPTY => panic!("Invalid turn value."),
    }

    return InputStatus::Success;
} 

fn print_input_status(status: InputStatus) {
    println!("\n");
    match status {
        InputStatus::NotTwoDigits  => eprint!("The given input is not a two digit number representing the row and column!"),
        InputStatus::RowNotBaseTen => eprint!("The given digit to row is not base 10."),
        InputStatus::ColNotBaseTen => eprint!("The given digit to column is not base 10."),
        InputStatus::InvalidRowVal => eprint!("Invalid value for row. Must be in range [1, 3]."),
        InputStatus::InvalidColVal => eprint!("Invalid value for column. Must be in range [1, 3]."),
        InputStatus::GridOccupied  => eprint!("The chosen grid is already occupied!"),
        InputStatus::Success       => panic!("This should be an invalid move!"),
    }

    eprintln!(" Try again!");
}

fn print_board(board: &Board, mapping: &HashMap<State, char>) {
    println!("   \t     COL\n");
    println!("   \t  1   2   3");

    println!("   \t1 {} ║ {} ║ {}", 
             mapping[&board[0][0]], 
             mapping[&board[0][1]], 
             mapping[&board[0][2]]
    );

    println!("   \t  ══╬═══╬══");

    println!("ROW\t2 {} ║ {} ║ {}", 
             mapping[&board[1][0]], 
             mapping[&board[1][1]], 
             mapping[&board[1][2]]
    );
 
    println!("   \t  ══╬═══╬══");   

    println!("   \t3 {} ║ {} ║ {}", 
             mapping[&board[2][0]], 
             mapping[&board[2][1]], 
             mapping[&board[2][2]]
    );
}

fn check_winner(board: &Board) -> State {
    // Check per-row win condition.
    for r in 0..ROWS {
        match board[r][..] {
            [State::X, State::X, State::X] => return State::X,
            [State::O, State::O, State::O] => return State::O,
            [..]                           => (),
        };
    }

    // Check per-column win condition.
    for c in 0..COLS {
        match board[..][c] {
            [State::X, State::X, State::X] => return State::X,
            [State::O, State::O, State::O] => return State::O,
            [..]                           => (),                        
        };
    }

    // Check the cross-diagonals win condition.
    let mut diag = [board[0][0], board[1][1], board[2][2]];

    match diag {
        [State::X, State::X, State::X] => return State::X,
        [State::O, State::O, State::O] => return State::O,
        [..]                           => (),                        
    };

    diag = [board[2][0], board[1][1], board[0][2]];

    match diag {
    [State::X, State::X, State::X] => return State::X,
    [State::O, State::O, State::O] => return State::O,
    [..]                           => (),                        
    };

    return State::EMPTY; 
}

fn clear_screen() {
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
