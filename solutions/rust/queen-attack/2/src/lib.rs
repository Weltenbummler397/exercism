#[derive(Debug)]
pub struct ChessPosition {
    column: i32,
    row: i32,
}

#[derive(Debug)]
pub struct Queen{
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..=7).contains(&rank) || !(0..=7).contains(&file) {
            None
        } else {
            Some(ChessPosition {
                column: file,
                row: rank,
            })
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        if self.position.row == other.position.row {
            true
        } else if self.position.column == other.position.column {
            true
        } else if (self.position.row - other.position.row).abs()
            == (self.position.column - other.position.column).abs() {
            true
        } else {
            false
        }
    }
}
