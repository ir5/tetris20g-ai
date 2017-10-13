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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CurrentPieceState {
    pub piece_type: u8,
    pub x: i8,
    pub y: i8,
    pub rotation: usize,
    pub first: bool,
}

pub enum Command {
    Move(i8, i8), // (dx, rotate)
    Fix,
}

pub enum CommandResult {
    Moved(CurrentPieceState, bool), // (new state, if lock delay is canceled)
    Fixed(Field, i8), // (new field, deleted lines)
    Ended,
}

pub fn apply_command(field: &Field, state: &CurrentPieceState, command: &Command) -> CommandResult {
    // TODO
    let piece_cycle = cycle(state.piece_type);
    let mut new_state = state.clone();
    if state.first {
        // first move: IRS is possible
        if let &Command::Move(_, rotate) = command {
            new_state.rotation = state.rotation + ((rotate + 4) as usize);
            new_state.rotation %= piece_cycle;
        }
        new_state.first = false;
        if let ValidityResult::Invalid(_) = check_validity(&field, &new_state) {
            return CommandResult::Ended;
        }
    } else if let &Command::Move(dx, rotate) = command {
        // standard move
        // rotation is first, position move is second
        let old_rotation = state.rotation;
        new_state.rotation = state.rotation + ((rotate + 4) as usize);
        new_state.rotation %= piece_cycle;

        let validity1 = check_validity(&field, &new_state);
        if let ValidityResult::Invalid(true) = validity1 {
            new_state.x += 1;
            if let ValidityResult::Invalid(_) = check_validity(&field, &new_state) {
                new_state.x -= 2;
                if let ValidityResult::Invalid(_) = check_validity(&field, &new_state) {
                    // wall kick failed. revert the state
                    new_state.x += 1;
                    new_state.rotation = old_rotation;
                }
            }
        } else if let ValidityResult::Invalid(false) = validity1 {
            // wall kick failed. revert the state
            new_state.rotation = old_rotation;
        }

        new_state.x += dx;
        if let ValidityResult::Invalid(_) = check_validity(&field, &new_state) {
            // movement is invalid. revert the state.
            new_state.x -= dx;
        }
    }

    // apply 20G drop
    new_state.y += 1;
    let mut reset = false;
    while let ValidityResult::Valid = check_validity(&field, &new_state) {
        new_state.y += 1;
        reset = true;
    }
    new_state.y -= 1;

    if let &Command::Fix = command {
        // fix current piece
        let mut new_field = field.clone();
        let sh = shape(new_state.piece_type, new_state.rotation);
        for (i, &row) in sh.iter().enumerate() {
            for (j, cell) in row.bytes().enumerate() {
                if cell == b'.' {
                    continue;
                }
                let y = (new_state.y + (i as i8)) as usize;
                let x = (new_state.x + (j as i8)) as usize;
                new_field[y][x] = new_state.piece_type;
            }
        }

        // delete lines
        let mut dels = [false; HEIGHT];
        let mut del = 0;
        for i in 0..HEIGHT {
            let n = new_field[i].iter().fold(0, |sum, &cell| {
                sum + if cell != b'.' { 1 } else { 0 }
            });
            if n == WIDTH {
                dels[i] = true;
                del += 1;
            }
        }
        let mut base = HEIGHT - 1;
        for i in (0..HEIGHT).rev() {
            if dels[i] {
                new_field[i] = [b'.'; WIDTH];
            } else {
                if i != base {
                    new_field.swap(i, base);
                }
                if base > 0 {
                    base -= 1;
                }
            }
        }

        CommandResult::Fixed(new_field, del)
    } else {
        CommandResult::Moved(new_state, reset)
    }
}

enum ValidityResult {
    Valid,
    Invalid(bool), // if further wall kick is possible
}

fn check_validity(field: &Field, state: &CurrentPieceState) -> ValidityResult {
    let mut ret = ValidityResult::Valid;
    let sh = shape(state.piece_type, state.rotation);
    for (i, &row) in sh.iter().enumerate() {
        for (j, cell) in row.bytes().enumerate() {
            if cell == b'.' {
                continue;
            }
            let y = state.y + i as i8;
            let x = state.x + j as i8;

            if y < 0 {
                continue;
            }

            if x < 0 || y as usize >= HEIGHT || x as usize >= WIDTH ||
                field[y as usize][x as usize] != b'.'
            {
                if (state.piece_type == b'L' || state.piece_type == b'J') && j == 1 {
                    return ValidityResult::Invalid(false);
                } else if state.piece_type == b'I' {
                    return ValidityResult::Invalid(false);
                } else {
                    ret = ValidityResult::Invalid(true);
                }
            }
        }
    }
    ret
}

pub fn new_piece(piece_type: u8) -> CurrentPieceState {
    CurrentPieceState {
        piece_type: piece_type,
        x: 3,
        y: -(y_offset(piece_type) as i8),
        rotation: 0,
        first: true,
    }
}

pub struct Game {
    pub field: Field,
    pub state: CurrentPieceState,
    pub piece_array: Vec<u8>,
    pub current_piece_id: usize,
}

impl Game {
    pub fn new(piece_array: Vec<u8>) -> Game {
        Game {
            field: EMPTY_FIELD,
            state: new_piece(piece_array[0]),
            piece_array,
            current_piece_id: 0,
        }
    }

    pub fn input(&mut self, key: char) {
        let command = match key {
            'z' => Some(Command::Move(-1, 0)),
            'x' => Some(Command::Fix),
            'c' => Some(Command::Move(1, 0)),
            'm' | '.' => Some(Command::Move(0, 1)),
            ',' => Some(Command::Move(0, -1)),
            _ => None,
        };

        if let Some(command) = command {
            let res = apply_command(&self.field, &self.state, &command);
            match res {
                CommandResult::Moved(next_state, _) => {
                    self.state = next_state;
                }
                CommandResult::Fixed(next_field, _) => {
                    self.field = next_field;
                    self.state = new_piece(self.next_piece().unwrap());
                    self.current_piece_id += 1;
                }
                CommandResult::Ended => {
                    panic!();
                }
            }
        }
    }

    pub fn next_piece(&self) -> Option<u8> {
        if self.current_piece_id + 1 < self.piece_array.len() {
            Some(self.piece_array[self.current_piece_id + 1])
        } else {
            None
        }
    }
}
