use game_instance::{Connect4GameInstance, GameInstance};

mod game_instance;

fn main() {
    let mut game_instance = Connect4GameInstance::new();

    game_instance.start();
}
