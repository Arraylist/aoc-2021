fn main() {
    let rows: Vec<&str> = include_str!("./input.txt")
        .trim()
        .split("\n")
        .collect();
    
    let mut matrix: Vec<Vec<u32>> = Vec::with_capacity(rows.len());

    for row in rows {
        let numbers: Vec<u32> = row
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        
        matrix.push(numbers);
    }

    println!("{}", part_1(&matrix));
}

fn part_1(matrix: &Vec<Vec<u32>>) -> u32 {
    let mut low_pts: Vec<u32>  = Vec::with_capacity(1000);
    for (i, row) in matrix.iter().enumerate() {
        for (j, low_pt) in row.iter().enumerate() {
            // first row
            if i == 0 {
                if j == 0 {
                    // first elem
                    if low_pt < right(&row, j) &&
                        low_pt < down(&matrix, i, j) {
                        low_pts.push(*low_pt);
                    }
                    // last elem
                } else if j == row.len() - 1 {
                    if low_pt < left(&row, j) &&
                        low_pt < down(&matrix, i, j) {
                        low_pts.push(*low_pt);
                    }
                } else {
                    if low_pt < left(&row, j) &&
                        low_pt < right(&row, j) &&
                        low_pt < down(&matrix, i, j) {
                        low_pts.push(*low_pt);
                    }
                }
            // last row
            } else if i == matrix.len() - 1 {
                if j == 0 {
                    // first elem
                    if low_pt < right(&row, j) &&
                        low_pt < up(&matrix, i, j) {
                        low_pts.push(*low_pt);
                    }
                    // last elem
                } else if j == row.len() - 1 {
                    if low_pt < left(&row, j) &&
                        low_pt < up(&matrix, i, j) {
                        low_pts.push(*low_pt);
                    }
                } else {
                    if low_pt < left(&row, j) &&
                        low_pt < right(&row, j) &&
                        low_pt < up(&matrix, i, j) {
                        low_pts.push(*low_pt);
                    }
                }
            } else {
                if j == 0 {
                    // first elem
                    if low_pt < right(&row, j) &&
                        low_pt < down(&matrix, i, j) &&
                        low_pt < up(&matrix, i, j) {
                        low_pts.push(*low_pt);
                    }
                    // last elem
                } else if j == row.len() - 1 {
                    if low_pt < left(&row, j) &&
                        low_pt < down(&matrix, i, j) &&
                        low_pt < up(&matrix, i, j) {
                        low_pts.push(*low_pt);
                    }
                } else {
                    if low_pt < left(&row, j) &&
                        low_pt < right(&row, j) &&
                        low_pt < down(&matrix, i, j) && 
                        low_pt < up(&matrix, i, j){
                        low_pts.push(*low_pt);
                    }
                }
            } 
        }  
    }

    return low_pts
        .into_iter()
        .fold(0, |accum, height| accum + height + 1);
}

fn right(v: &Vec<u32>, i: usize) -> &u32 {
    return &v[i + 1];
}

fn left(v: &Vec<u32>, i: usize) -> &u32 {
    return &v[i - 1];
}

fn down(m: &Vec<Vec<u32>>, i: usize, j: usize) -> &u32 {
    return &m[i + 1][j];
}

fn up(m: &Vec<Vec<u32>>, i: usize, j: usize) -> &u32 {
    return &m[i - 1][j];
}

