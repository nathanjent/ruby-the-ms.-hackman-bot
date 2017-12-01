use std::fmt;

#[derive(Debug)]
pub struct Player {
    name: String,
    bombs: i32,
    snippets: i32,
    character: CharacterType,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name: name,
            bombs: 0,
            snippets: 0,
            character: CharacterType::Bixie,
        }
    }

    pub fn character_type(&self) -> String {
        match self.character {
            CharacterType::Bixie => format!("bixie"),
            CharacterType::Bixiette => format!("bixiette"),
        }
    }
}

#[derive(Debug)]
pub enum CharacterType {
    Bixie,
    Bixiette,
}

impl CharacterType {}

#[derive(Debug)]
pub struct Move {
    move_type: MoveType,
    bomb_ticks: i32,
}

#[derive(Debug)]
pub enum MoveType {
    Up,
    Down,
    Left,
    Right,
    Pass,
}

impl Move {
    pub fn new() -> Self {
        Move {
            move_type: MoveType::Pass,
            bomb_ticks: 0,
        }
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let move_type = match self.move_type {
            MoveType::Up => "up",
            MoveType::Down => "down",
            MoveType::Left => "left",
            MoveType::Right => "right",
            MoveType::Pass => "pass",
        };
        write!(f, "{};drop_bomb {}", move_type, self.bomb_ticks)
    }
}
