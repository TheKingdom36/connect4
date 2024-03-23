use game_board::Color;
use game_board::Connect4GameBoard;

use text_io::read;

mod game_board;

pub struct Player {
    pub color: Color,
}

pub struct Connect4GameInstance {
    pub player_a: Player,
    pub player_b: Player,
    pub game_board: Connect4GameBoard,
}

pub trait GameInstance {
    fn new() -> Self;
    fn start(&mut self) -> ();
}

impl GameInstance for Connect4GameInstance {
    fn new() -> Self {
        return Connect4GameInstance {
            player_a: Player {
                color: Color::Yellow,
            },
            player_b: Player { color: Color::Red },
            game_board: Connect4GameBoard {
                board: [[Color::Empty; 7]; 6],
            },
        };
    }

    fn start(&mut self) -> () {
        let mut game_ended;
        let mut active_player = &self.player_a;

        println!("The game has started!");
        println!("{:?}", self.game_board);

        loop {
            let mut selected_col: usize;
            loop {
                selected_col = read!();

                if self.game_board.available_columns().contains(&selected_col) {
                    break;
                } else {
                    println!("Please select a valid column");
                };
            }
            game_ended = self
                .game_board
                .insert_coin(selected_col, active_player.color);

            println!("{:?}", self.game_board);
            if game_ended {
                print!("Game has ended you should feel good");
                return;
            }

            if active_player.color == self.player_a.color {
                active_player = &self.player_b
            } else {
                active_player = &self.player_a
            }
        }
    }
}
