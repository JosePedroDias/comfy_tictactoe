use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Marker {
    X,
    O,
    NA,
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match self {
           Marker::X => write!(f, "X"),
           Marker::O => write!(f, "O"),
           Marker::NA => write!(f, " "),
       }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Board {
    pub spaces: [Marker; 9],
    pub current_player: Marker,
}

pub fn new_board() -> Board {
    Board { 
        spaces: [Marker::NA; 9],
        current_player: Marker::X,
    }
}

pub fn get_other_player(pl: Marker) -> Marker {
    if pl == Marker::X { Marker::O } else { Marker:: X }
}

fn count_cells_of_type(board: &Board, typ: Marker) -> usize {
    let mut count: usize = 0;
    for el in board.spaces.iter() {
        if *el == typ {
            count += 1;
        }
    }
    count
}

const DIRECTIONS: [[usize; 3]; 8] = [
    // rows
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    //cols
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    //diagonals
    [0, 4, 8],
    [6, 4, 2],
];

pub fn who_won(board: &Board) -> Option<Marker> {
    for marker in [Marker::X, Marker::O] {
        'trio:
        for trio in DIRECTIONS {
            for i in trio {
                if board.spaces[i] != marker {
                    continue 'trio;
                }
            }
            return Some(marker);
        }
    }
    None
}

pub fn is_full(board: &Board) -> bool {
    count_cells_of_type(board, Marker::NA) == 0
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for (i, v) in self.spaces.iter().enumerate() {
            //s.push('x');
            let ch = match v {
                Marker::X => 'X',
                Marker::O => 'O',
                Marker::NA => ' ',
            };
            s.push(ch);
            if i % 3 == 2 {
                s.push('\n');
            }
        }
        write!(f, "{s}")
    }
}
