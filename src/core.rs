const HEIGHT: usize = 20;
const WIDTH: usize = 10;

pub type Field = [[u8; WIDTH]; HEIGHT];

pub const EMPTY_FIELD: Field = [[b'.'; WIDTH]; HEIGHT];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn shape(piece_type: u8, rotation: usize) -> [&'static str; 4] {
    match (piece_type, rotation) {
        (b'I', 0) => ["....",
                      "####",
                      "....",
                      "...."],
        (b'I', 1) => ["..#.",
                      "..#.",
                      "..#.",
                      "..#."],

        (b'O', 0) => ["....",
                      "....",
                      ".##.",
                      ".##."],

        (b'S', 0) => ["....",
                      "....",
                      ".##.",
                      "##.."],
        (b'S', 1) => ["....",
                      "#...",
                      "##..",
                      ".#.."],

        (b'Z', 0) => ["....",
                      "....",
                      "##..",
                      ".##."],
        (b'Z', 1) => ["....",
                      "..#.",
                      ".##.",
                      ".#.."],

        (b'L', 0) => ["....",
                      "....",
                      "###.",
                      "#..."],
        (b'L', 1) => ["....",
                      ".#..",
                      ".#..",
                      ".##."],
        (b'L', 2) => ["....",
                      "....",
                      "..#.",
                      "###."],
        (b'L', 3) => ["....",
                      "##..",
                      ".#..",
                      ".#.."],

        (b'J', 0) => ["....",
                      "....",
                      "###.",
                      "..#."],
        (b'J', 1) => ["....",
                      ".##.",
                      ".#..",
                      ".#.."],
        (b'J', 2) => ["....",
                      "....",
                      "#...",
                      "###."],
        (b'J', 3) => ["....",
                      ".#..",
                      ".#..",
                      "##.."],

        (b'T', 0) => ["....",
                      "....",
                      "###.",
                      ".#.."],
        (b'T', 1) => ["....",
                      ".#..",
                      ".##.",
                      ".#.."],
        (b'T', 2) => ["....",
                      "....",
                      ".#..",
                      "###."],
        (b'T', 3) => ["....",
                      ".#..",
                      "##..",
                      ".#.."],
        _ => panic!(),
    }
}

pub fn y_offset(piece_type: u8) -> usize {
    match piece_type {
        b'I' => 1,
        b'O' => 2,
        b'S' => 2,
        b'Z' => 2,
        b'L' => 2,
        b'J' => 2,
        b'T' => 2,
        _ => panic!(),
    }
}

pub fn cycle(piece_type: u8) -> usize {
    match piece_type {
        b'I' => 2,
        b'O' => 1,
        b'S' => 2,
        b'Z' => 2,
        b'L' => 4,
        b'J' => 4,
        b'T' => 4,
        _ => panic!(),
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct CurrentPieceState {
    pub piece_type: u8,
    pub x: i8,
    pub y: i8,
    pub rotation: usize,
}

pub enum Command {
    Move(i8, i8), // (dx, rotate)
    Fix,
}

pub enum CommandResult {
    Move(CurrentPieceState, bool), // (new state, if lock delay is canceled)
    Fixed(Field, i8), // (new field, deleted lines)
}

pub fn apply_command(field: &Field, state: &CurrentPieceState, command: &Command) -> CommandResult {
    // TODO
    CommandResult::Fixed(EMPTY_FIELD, 0)
}

pub fn new_piece(field: &Field, piece_type: u8) -> Option<CurrentPieceState> {
    // TODO
    None
}

pub struct Game {
    pub field: Field,
    pub state: CurrentPieceState,
    pub piece_array: Vec<u8>,
    pub current_piece_id: usize,
}


