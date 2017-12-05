use player::Player;
use field::Field;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct BotState {
    pub settings: Rc<RefCell<Settings>>,
    pub players: Rc<RefCell<HashMap<String, Player>>>,
    pub field: Rc<RefCell<Field>>,
}

#[derive(Debug)]
pub struct Settings {
    pub name: String,
    pub id: i32,
    pub time_bank: i32,
    pub time_per_move: i32,
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
                round: 0,
                max_rounds: 0,
                opponent_name: String::new(),
            })),
            players: Rc::new(RefCell::new(HashMap::new())),
            field: Rc::new(RefCell::new(Field::new(1, 1))),
        }
    }
}
