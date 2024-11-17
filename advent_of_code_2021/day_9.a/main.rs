fn main() {
    // Input parsing
    let input = include_str!("./input.txt");
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut risk_sum = 0;

    // Iterate over every cell in the grid
    for row in 0..rows {
        for col in 0..cols {
            let current = grid[row][col];

            // Check all four neighbors, accounting for boundaries
            let up = row == 0 || current < grid[row - 1][col];
            let down = row == rows - 1 || current < grid[row + 1][col];
            let left = col == 0 || current < grid[row][col - 1];
            let right = col == cols - 1 || current < grid[row][col + 1];

            // If all conditions are satisfied, it's a low point
            if up && down && left && right {
                risk_sum += current + 1;
            }
        }
    }

    println!("Sum of the risk levels of all low points: {}", risk_sum);
}

