use player::Player;
use field::Field;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct BotState {
    pub settings: Rc<RefCell<Settings>>,
    pub player: Rc<RefCell<Player>>,
    pub field: Rc<RefCell<Field>>,
}

#[derive(Debug)]
pub struct Settings {
    pub name: String,
    pub id: i32,
    pub time_bank: i32,
    pub time_per_move: i32,
    pub players: HashMap<String, Player>,
    pub round: i32,
    pub max_rounds: i32,
    pub opponent_name: String,
}

impl BotState {
    pub fn new() -> Self {
        BotState {
            settings: Rc::new(RefCell::new(Settings {
                name: String::new(),
                id: 0,
                time_bank: 0,
                time_per_move: 0,
                players: HashMap::new(),
                round: 0,
                max_rounds: 0,
                opponent_name: String::new(),
            })),
            player: Rc::new(RefCell::new(Player::new("".into()))),
            field: Rc::new(RefCell::new(Field::new(1, 1))),
        }
    }
}
