use simple_matrix::Matrix;
use error::{Error, ParseErrorKind, Result};

#[derive(Debug)]
pub struct Field {
    pub player_id: String,
    pub opponent_id: String,
    pub field: Matrix<Cell>,
    pub player_position: Option<Point>,
    pub opponent_position: Option<Point>,
    pub enemy_positions: Vec<Point>,
    pub snippet_positions: Vec<Point>,
    pub bomb_positions: Vec<Point>,
    pub ticking_bomb_positions: Vec<Point>,
}

#[derive(Clone, Debug)]
pub struct Cell {
    cell_items: Vec<CellItem>,
}

#[derive(Clone, Copy, Debug)]
pub enum CellItem {
    Empty,
    Inaccessible,
    Player(i32),
    Spawn(Option<i32>),
    Gate(GateDirection),
    Enemy(AiType),
    Bomb(i32),
    CodeSnippet,
}

#[derive(Clone, Copy, Debug)]
pub enum GateDirection {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum AiType {
    Chase,
    Predict,
    Lever,
    FarChase,
}

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
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

    pub fn update_field(&mut self, field: Vec<Cell>) {
        self.field.m = field;
    }

    pub fn set_width(&mut self, width: usize) {
        self.field.width = width;
    }

    pub fn set_height(&mut self, height: usize) {
        self.field.height = height;
    }
}

impl Cell {
    pub fn new() -> Cell {
        Cell { cell_items: Vec::new() }
    }
}

pub fn parse_field(s: &str) -> Result<Vec<Cell>> {
    s.split(",")
        .map(|cell| {
            cell.split(";")
                .map(|cell_type| cell_type.parse::<CellItem>())
                .collect()
        })
        .collect()
}

impl ::std::iter::FromIterator<CellItem> for Cell {
    fn from_iter<I: IntoIterator<Item = CellItem>>(iter: I) -> Self {
        let mut c = Cell::new();
        for i in iter {
            c.cell_items.push(i);
        }
        c
    }
}

impl ::std::str::FromStr for CellItem {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let cell_type = s.split_at(1);
        match cell_type {
            (".", _) => Ok(CellItem::Empty),
            ("x", _) => Ok(CellItem::Inaccessible),
            ("P", v) => {
                // Player<id>
                match v.parse::<i32>() {
                    Ok(n) => Ok(CellItem::Player(n)),
                    Err(e) => Err(Error::ParseError(ParseErrorKind::NumberFormat(Box::new(e)))),
                }
            }
            ("S", "") => Ok(CellItem::Spawn(None)),
            ("S", v) => {
                // Bug spawn point<rounds_until>
                match v.parse::<i32>() {
                    Ok(n) => Ok(CellItem::Spawn(Some(n))),
                    Err(e) => Err(Error::ParseError(ParseErrorKind::NumberFormat(Box::new(e)))),
                }
            }
            ("G", "l") => Ok(CellItem::Gate(GateDirection::Left)),
            ("G", "r") => Ok(CellItem::Gate(GateDirection::Right)),

            // Enemy<bug_ai_type>
            ("E", "0") => Ok(CellItem::Enemy(AiType::Chase)),
            ("E", "1") => Ok(CellItem::Enemy(AiType::Predict)),
            ("E", "2") => Ok(CellItem::Enemy(AiType::Lever)),
            ("E", "3") => Ok(CellItem::Enemy(AiType::FarChase)),
            ("B", v) => {
                // Bomb<rounds_until>
                match v.parse::<i32>() {
                    Ok(n) => Ok(CellItem::Bomb(n)),
                    Err(e) => Err(Error::ParseError(ParseErrorKind::NumberFormat(Box::new(e)))),
                }
            }
            ("C", "") => {
                // Code Snippet
                Ok(CellItem::CodeSnippet)
            }
            _ => Err(Error::ParseError(ParseErrorKind::InvalidCellType)),
        }
    }
}

#[cfg(test)]
mod Test {
    use field::{parse_field, CellItem, AiType, GateDirection};
    #[test]
    fn parse_field_test() {
        let field_str = ".,x,P0,S,S2,Gl,E0,B3,C";

        let expected_cells = vec![CellItem::Empty,
                                  CellItem::Inaccessible,
                                  CellItem::Player(0),
                                  CellItem::Spawn(None),
                                  CellItem::Spawn(Some(2)),
                                  CellItem::Gate(GateDirection::Left),
                                  CellItem::Enemy(AiType::Chase),
                                  CellItem::Bomb(3),
                                  CellItem::CodeSnippet];
        let actual_cells = parse_field(field_str);
    }
}
