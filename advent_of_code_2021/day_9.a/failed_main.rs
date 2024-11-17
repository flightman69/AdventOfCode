fn main() {
    let file = include_str!("./input.txt");

    let n: Vec<Vec<i32>> = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|n| n.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();
    let last_row = n.len();
    let last_col = n[0].len();

    let mut sol = 0;
    for row in 1..last_row {
        for col in 1..last_col {
            let no = n[row][col];
            if no < n[row][col - 1] // left
                && no < n[row][col + 1] // right
                && no < n[row - 1][col] // up 
                && no < n[row + 1][col] // down
                && no < n[row - 1][col - 1] // top left corner
                && no < n[row - 1][col + 1] // top right corner 
                && no < n[row + 1][col + 1] // bottom right corner
                && no < n[row + 1][col - 1]
            // bottom left corner
            {
                sol += n[row][col] + 1;
            }
        }
    }

    // For all corner elements
    if n[0][0] < n[0][1] && n[0][0] < n[1][0] && n[0][0] < n[1][1] {
        sol += n[0][0] + 1;
    }
    if n[last_row][last_col] < n[last_row][last_col - 1]
        && n[last_row][last_col] < n[last_row - 1][last_col]
        && n[last_row][last_col] < n[last_row - 1][last_col - 1]
    {
        sol += n[last_row][last_col] + 1;
    }
    if n[0][last_col] < n[0][last_col - 1]
        && n[0][last_col] < n[1][last_col]
        && n[0][last_col] < n[1][last_col - 1]
    {
        sol += n[0][last_col] + 1;
    }
    if n[last_row][0] < n[last_row][1]
        && n[last_row][0] < n[last_row - 1][0]
        && n[last_row][0] < n[last_row - 1][1]
    {
        sol += n[last_row][0] + 1;
    }

    // for top row
    for i in 1..last_col {
        if n[0][i] < n[0][i - 1] // left
            && n[0][i] < n[0][i + 1] // right 
            && n[0][i] < n[1][i] //bottom 
            && n[0][i] < n[1][i - 1] // bottom left 
            && n[0][i] < n[1][i + 1]
        // bottom right
        {
            sol += n[0][i] + 1;
        }
    }

    // for left row
    for i in 1..last_row {
        if n[i][0] < n[i][1]
            && n[i][0] < n[i - 1][0]
            && n[i][0] < n[i + 1][0]
            && n[i][0] < n[i - 1][1]
            && n[i][0] < n[i + 1][1]
        {
            sol += n[i][0] + 1;
        }
    }

    // for bottom row
    for i in 1..last_col {
        if n[last_row][i] < n[last_row][i - 1]
            && n[last_row][i] < n[last_row][i + 1]
            && n[last_row][i] < n[last_row - 1][i]
            && n[last_row][i] < n[last_row - 1][i - 1]
            && n[last_row][i] < n[last_row - 1][i + 1]
        {
            sol += n[last_row][i] + 1;
        }
    }

    // for right row
    for i in 1..last_row {
        if n[i][last_col] < n[i][last_col - 1]
            && n[i][last_col] < n[i - 1][last_col]
            && n[i][last_col] < n[i + 1][last_col]
            && n[i][last_col] < n[i - 1][last_col - 1]
            && n[i][last_col] < n[i + 1][last_col - 1]
        {
            sol += n[i][last_col] + 1;
        }
    }

    println!("{}", sol);
}
