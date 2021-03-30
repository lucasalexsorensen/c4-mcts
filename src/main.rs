use std::{io::{Write, stdin, stdout}};
use crate::board::Board;
use crate::solver::Solver;
mod board;
mod draw;
mod solver;

fn print_board (board: &Board) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", board);
}

fn prompt_player_move () -> u8 {
    let mut s = String::new();
    print!("Enter move (1-7): ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("STDIN error!");
    if let Some('\n') = s.chars().next_back() { s.pop(); }
    if let Some('\r') = s.chars().next_back() { s.pop(); }
    return s.parse().expect("Parser error!");
}

fn main() {
    let mut board = Board::from(vec![
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0
    ]);
    print_board(&board);

    let mut solver = Solver::default();
    loop {
        if board.moves_count % 2 == 0 { // agent's turn
            let result = solver.search(board);
            board = board.into_move(result);
        } else { // agent's turn
            let player_move = prompt_player_move();
            board = board.into_move(player_move - 1);
        }

        print_board(&board);
        if board.is_game_over() {
            println!("GAME OVER! {} wins", if board.moves_count % 2 == 1 { "PLAYER" } else { "AGENT" });
            break;
        }
    }
}
