pub struct Hook {
    pub rotating: [[[bool; 4]; 4]; 4],
    pub hook_size: usize,
    pub num_piace: u64,
}

const HOOKMARK: [[bool; 4]; 4] = [
    [false, true, false, false],
    [true, false, true, true],
    [true, false, false, false],
    [false, false, false, false],
];

impl Hook {
    pub const fn new() -> Self {
        let mut rotating = [[[false; 4]; 4]; 4];
        rotating[0] = HOOKMARK;
        let mut rotate = 1;
        let mut y;
        let mut x;
        while rotate < 4 {
            y = 0;
            while y < 4 {
                x = 0;
                while x < 4 {
                    rotating[rotate][y][x] = rotating[rotate - 1][3 - x][y];
                    x += 1;
                }
                y += 1;
            }
            rotate += 1;
        }
        Self {
            rotating,
            hook_size: 4,
            num_piace: 5,
        }
    }
}
