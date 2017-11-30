use player::Player;
use field::Field;
use std::collections::HashMap;
pub struct BotState {
    field: Field,
    players: HashMap<String, Player>,
    round: i32,
    time_bank: i32,
    name: String,
}
