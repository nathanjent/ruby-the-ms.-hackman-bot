use std::ops::{Index, IndexMut};
use std::str::FromStr;

pub struct Field {
    player_id: String,
    opponent_id: String,
    field: Matrix<Location>,
    player_position: Option<Point>,
    opponent_position: Option<Point>,
    enemy_positions: Vec<Point>,
    snippet_positions: Vec<Point>,
    bomb_positions: Vec<Point>,
    ticking_bomb_positions: Vec<Point>,
}

pub struct FieldUpdate {}

pub struct Matrix<T> {
    m: Vec<T>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy)]
pub enum Location {
    Empty,
    Blocked,
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl<T> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Matrix {
            m: Vec::new(),
            width: width,
            height: height,
        }
    }

    pub fn reset_to(&mut self, value: T)
        where T: Copy
    {
        for v in self.m.iter_mut() {
            *v = value;
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, i: (usize, usize)) -> &T {
        &self.m[i.0 * self.width + i.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, i: (usize, usize)) -> &mut T {
        &mut self.m[i.0 * self.width + i.1]
    }
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        Field {
            player_id: String::new(),
            opponent_id: String::new(),
            field: Matrix::new(width, height),
            player_position: None,
            opponent_position: None,
            enemy_positions: Vec::new(),
            snippet_positions: Vec::new(),
            bomb_positions: Vec::new(),
            ticking_bomb_positions: Vec::new(),
        }
    }

    pub fn clear_field(&mut self) {
        self.field.reset_to(Location::Empty);
    }
}

pub enum FieldCell {
    Player(i32),
    Spawn,
    Enemy(EnemyType),
    Bomb,
    CodeSnippet,
}

pub enum EnemyType {
}

//impl FromStr for FieldUpdate {
//    type Err = ParseError;
//
//    fn from_str(s: &str) -> Result<Self, Self::Err> {
//        s.split(",").map(|cell| {
//            cell.split(":")
//                .flat_map(|p| {
//                    match p {
//                        "P" => {
//                            // Player<id>
//                        }
//                        "e" => {
//                            // Spawn point
//                        }
//                        "E" => {
//                            // Enemy
//                        }
//                        "B" => {
//                            // Bomb
//                        }
//                        "C" => {
//                            // Code Snippet
//                        }
//                        _ => None,
//                    }
//                })
//            .collect()
//        })
//        .collect()
//
//    }
//}
