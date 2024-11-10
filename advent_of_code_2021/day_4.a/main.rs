use std::fs;

#[derive(Debug, Clone)]
struct Board {
    grid: Vec<Vec<Option<u32>>>, // Use Option<u32> to mark drawn numbers as None
}

impl Board {
    fn new(grid: Vec<Vec<u32>>) -> Self {
        let grid = grid
            .into_iter()
            .map(|row| row.into_iter().map(Some).collect())
            .collect();
        Self { grid }
    }

    // Mark a number on the board
    fn mark_number(&mut self, number: u32) {
        for row in &mut self.grid {
            for cell in row {
                if *cell == Some(number) {
                    *cell = None; // Mark it as None to indicate it's been drawn
                }
            }
        }
    }

    // Check if the board has a winning row or column
    fn is_winning(&self) -> bool {
        // Check rows
        if self
            .grid
            .iter()
            .any(|row| row.iter().all(|&cell| cell.is_none()))
        {
            return true;
        }
        // Check columns
        for col in 0..5 {
            if (0..5).all(|row| self.grid[row][col].is_none()) {
                return true;
            }
        }

        if (0..5).all(|i| self.grid[i][i].is_none()) {
            return true;
        }

        if (0..5).all(|i| self.grid[i][4 - i].is_none()) {
            return true;
        }
        false
    }

    // Calculate the score (sum of unmarked numbers)
    fn unmarked_sum(&self) -> u32 {
        self.grid.iter().flatten().filter_map(|&cell| cell).sum()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input");
    let mut sections = input.split("\n\n");

    // Parse drawn numbers
    let drawn_numbers: Vec<u32> = sections
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    // Parse boards
    let mut boards: Vec<Board> = sections
        .map(|board_text| {
            let grid = board_text
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| num.parse().unwrap())
                        .collect()
                })
                .collect();
            Board::new(grid)
        })
        .collect();

    // Play the game
    for number in drawn_numbers {
        for board in &mut boards {
            board.mark_number(number);
            if board.is_winning() {
                let score = board.unmarked_sum() * number;
                println!("Winning board score: {}", score);
                return;
            }
        }
    }
}

