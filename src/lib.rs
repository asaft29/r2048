pub mod decoration {

    use ratatui::style::Color;

    pub fn get_background_color(value: u32) -> Color {
        match value {
            2 => Color::Rgb(238, 228, 218),   // #eee4da
            4 => Color::Rgb(237, 224, 180),   // #ede0b4
            8 => Color::Rgb(242, 177, 121),   // #f2b179
            16 => Color::Rgb(245, 149, 99),   // #f59563
            32 => Color::Rgb(246, 124, 95),   // #f67c5f
            64 => Color::Rgb(246, 94, 59),    // #f65e3b
            128 => Color::Rgb(237, 207, 114), // #edcf72
            256 => Color::Rgb(237, 204, 65),  // #edcc41
            512 => Color::Rgb(231, 170, 40),  // #e7aa28
            1024 => Color::Rgb(214, 166, 0),  // #d6a600
            2048 => Color::Rgb(215, 149, 43), // #d7952b
            _ => Color::Black,
        }
    }
}
pub mod game_logic {
    use rand::seq::IteratorRandom;
    use std::cell::Cell;
    pub struct Board {
        pub size: [[u32; 4]; 4],
        pub high_score: Cell<u32>,
    }

    pub enum State {
        Menu,
        Playing,
        Lost,
        Won,
    }

    impl Board {
        pub fn new() -> Self {
            Self {
                size: [[0; 4]; 4],
                high_score: Cell::new(0),
            }
        }

        pub fn clear(&mut self) {
            self.size
                .iter_mut()
                .flat_map(|r| r.iter_mut())
                .for_each(|x| *x = 0);
        }
        pub fn init_board(&mut self) {
            self.clear();

            let mut rng = rand::rng();

            let empty_positions: Vec<(usize, usize)> = (0..4)
                .flat_map(|row| (0..4).map(move |col| (row, col)))
                .collect();

            for &(row, col) in empty_positions.iter().choose_multiple(&mut rng, 2).iter() {
                self.size[*row][*col] = 2;
            }
        }
        pub fn move_all_down(&mut self) {
            for i in 0..self.size.len() {
                let mut stack: Vec<(u32, usize)> = Vec::new();
                let mut j = 0;
                while j < self.size[0].len() {
                    stack.push((self.size[j][i], j));
                    j += 1;
                }
                while let Some(value) = stack.pop() {
                    let mut index = value.1;
                    while index + 1 < j {
                        let val: u32 = self.size[index][i];

                        match self.size[index + 1][i] {
                            0 => {
                                self.size[index + 1][i] = val;
                                self.size[index][i] = 0;
                                index += 1;
                            }

                            other if other == val => {
                                self.size[index + 1][i] += other;
                                self.size[index][i] = 0;
                                break;
                            }

                            _ => break,
                        }
                    }
                }
            }
        }

        pub fn move_all_up(&mut self) {
            for i in 0..self.size.len() {
                let mut stack: Vec<(u32, usize)> = Vec::new();
                let mut j = self.size[0].len() - 1;
                while j > 0 {
                    stack.push((self.size[j][i], j));
                    j -= 1;
                }
                while let Some(value) = stack.pop() {
                    let mut index = value.1;
                    while index > 0 {
                        let val: u32 = self.size[index][i];
                        match self.size[index - 1][i] {
                            0 => {
                                self.size[index - 1][i] = val;
                                self.size[index][i] = 0;
                                index -= 1;
                            }

                            other if other == val => {
                                self.size[index - 1][i] += other;
                                self.size[index][i] = 0;
                                break;
                            }

                            _ => break,
                        }
                    }
                }
            }
        }
        pub fn move_all_right(&mut self) {
            for i in 0..self.size.len() {
                let mut stack: Vec<(u32, usize)> = Vec::new();
                let mut j = 0;
                while j < self.size[0].len() {
                    stack.push((self.size[i][j], j));
                    j += 1;
                }
                while let Some(value) = stack.pop() {
                    let mut index = value.1;
                    while index + 1 < j {
                        let val = self.size[i][index];

                        match self.size[i][index + 1] {
                            0 => {
                                self.size[i][index + 1] = val;
                                self.size[i][index] = 0;
                                index += 1;
                            }
                            other if other == val => {
                                self.size[i][index + 1] += other;
                                self.size[i][index] = 0;
                                break;
                            }

                            _ => break,
                        }
                    }
                }
            }
        }

        pub fn move_all_left(&mut self) {
            for i in 0..self.size.len() {
                let mut stack: Vec<(u32, usize)> = Vec::new();
                let mut j = self.size[0].len() - 1;
                while j > 0 {
                    stack.push((self.size[i][j], j));
                    j -= 1;
                }
                while let Some(value) = stack.pop() {
                    let mut index = value.1;
                    while index > 0 {
                        let val = self.size[i][index];

                        match self.size[i][index - 1] {
                            0 => {
                                self.size[i][index - 1] = val;
                                self.size[i][index] = 0;
                                index -= 1;
                            }
                            other if other == val => {
                                self.size[i][index - 1] += other;
                                self.size[i][index] = 0;
                                break;
                            }

                            _ => break,
                        }
                    }
                }
            }
        }
        pub fn spawn_one_random(&mut self) {
            let mut rng = rand::rng();
            let mut empty_cells = Vec::new();

            for row in 0..4 {
                for col in 0..4 {
                    if self.size[row][col] == 0 {
                        empty_cells.push((row, col));
                    }
                }
            }

            if let Some((row, col)) = empty_cells.into_iter().choose(&mut rng) {
                let value = if rand::random::<f32>() < 0.9 { 2 } else { 4 };
                self.size[row][col] = value;
            }
        }
        pub fn won(&self) -> bool {
            self.size.iter().flatten().any(|&x| x == 2048)
        }

        pub fn lost(&self) -> bool {
            let n = self.size.len();

            if self.size.iter().flatten().any(|&x| x == 0) {
                return false;
            }

            for i in 0..n {
                for j in 0..n {
                    let current = self.size[i][j];
                    if j + 1 < n && self.size[i][j + 1] == current {
                        return false;
                    }
                    if i + 1 < n && self.size[i + 1][j] == current {
                        return false;
                    }
                }
            }

            true
        }

        pub fn calculate_score(&self) -> u32 {
            let val: u32 = self.size.iter().flatten().sum();
            self.high_score.set(val);
            val
        }
    }
}
