pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod hook;

const BOARD_WIDTH: usize = 20;
const BOARD_HEIGHT: usize = 20;

type Piace = u8; // 0: empty, 1: first, 2: second

pub struct HookMark {
    pub board: [[Piace; BOARD_WIDTH]; BOARD_HEIGHT],
    pub is_first: bool,
    board_is_first: [[bool; BOARD_WIDTH]; BOARD_HEIGHT],
    board_is_second: [[bool; BOARD_WIDTH]; BOARD_HEIGHT],
    board_position_value: [[f64; BOARD_WIDTH]; BOARD_HEIGHT],
    pub width: usize,
    pub height: usize,
}

const HOOK: hook::Hook = hook::Hook::new();

impl HookMark {
    pub fn new() -> Self {
        Self {
            board: [[0; BOARD_WIDTH]; BOARD_HEIGHT],
            is_first: true,
            board_is_first: [[false; BOARD_WIDTH]; BOARD_HEIGHT],
            board_is_second: [[false; BOARD_WIDTH]; BOARD_HEIGHT],
            board_position_value: [[0.0; BOARD_WIDTH]; BOARD_HEIGHT],
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT
        }
    }

    pub fn put(&mut self, x: usize, y: usize) {
        self.board[y][x] = if self.is_first { 1 } else { 2 };
        if self.is_first { 
            self.board_is_first[y][x] = true;
        } else {
            self.board_is_second[y][x] = true;
        }
        self.is_first = !self.is_first;
    }

    pub fn put_position_value(&mut self, x: usize, y: usize, value: f64) {
        self.board_position_value[y][x] = value;
    }

    pub fn get_position_value(&self, x: usize, y: usize) -> f64 {
        self.board_position_value[y][x]
    }

    pub fn get(&self, x: usize, y: usize) -> Piace {
        self.board[y][x]
    }

    pub fn get_is_first(&self, x: usize, y: usize) -> bool {
        self.board_is_first[y][x]
    }

    pub fn get_is_second(&self, x: usize, y: usize) -> bool {
        self.board_is_second[y][x]
    }

    pub fn is_first(&self) -> bool {
        self.is_first
    }

    pub fn is_second(&self) -> bool {
        !self.is_first
    }

    pub fn is_empty(&self, x: usize, y: usize) -> bool {
        self.board[y][x] == 0
    }

    pub fn is_full(&self) -> bool {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if self.board[y][x] == 0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_how_match_hook(&self,x: usize,y: usize, rotate: usize, is_first: bool) -> u64 {
        //gard index over width and height length of board
        if self.width <= x + HOOK.hook_size - 1 || self.height <= y + HOOK.hook_size - 1 { return 0; }
        let check_goihsi: Piace = if is_first { 1 } else { 2 };
        let mut result = 0;
        for hook_y in 0..4 {
            for hook_x in 0..4 {
                if HOOK.rotating[rotate % 4][hook_y][hook_x] && check_goihsi == self.board[y + hook_y][x + hook_x] {
                    result += 1;
                }
            }
        }
        result
    }

    pub fn is_the_player_win(&self,is_first: bool) -> bool {
        for y in 0..self.width - ( HOOK.hook_size - 1) {
            for x in 0..self.height - ( HOOK.hook_size - 1) {
                for rotate in 0..4 {
                    if HOOK.num_piace == self.get_how_match_hook(x,y,rotate,is_first) { return true; }
                }
            }
        }
        false
    }
}
