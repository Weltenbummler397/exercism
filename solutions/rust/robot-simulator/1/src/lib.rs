// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot{
    x: i32,
    y: i32,
    d: Direction
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self{x,y,d}
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let mut robot = self;
        robot.d = match robot.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        robot
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let mut robot = self;
        robot.d = match robot.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        robot
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let mut robot = self;
        match robot.d {
            Direction::North => robot.y+=1,
            Direction::East => robot.x+=1,
            Direction::South => robot.y-=1,
            Direction::West => robot.x-=1,
        };
        robot
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for t in instructions.chars() {
            robot = match t{
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => robot,
            };
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
