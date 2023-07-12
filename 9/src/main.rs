use std::hash::{Hash};
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let rows: Vec<&str> = include_str!("./exampleInput.txt")
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

    let (sum, coords) = part_1(&matrix);
    println!("{}", sum);
    println!("{}", part_2(&matrix, &coords));

}

fn part_2(matrix: &Vec<Vec<u32>>, coords: &Vec<(usize, usize)>) -> u32 {
    let t_mat: Vec<Vec<u32>> = transpose(matrix.to_vec());
    let visited: &mut HashSet<(usize, usize)> = &mut HashSet::new();
    let basins: &mut HashMap<(usize, usize), Vec<(usize, usize)>> = &mut HashMap::new();

    for low_pt_coord in coords {
        let row_idx = low_pt_coord.0;
        let col_idx = low_pt_coord.1;
        basins.insert(*low_pt_coord, Vec::with_capacity(1000));
        visited.insert(*low_pt_coord);
        if row_idx == 0 {
            // first
            if col_idx == 0 {
                let row = &matrix[row_idx];
                add_right((row, visited, basins, *low_pt_coord));

                let col = &t_mat[row_idx];
                add_bottom((col, visited, basins, *low_pt_coord));
            // last
            } else if col_idx == matrix[row_idx].len() - 1 {
                let row = &matrix[row_idx];
                add_left((row, visited, basins, *low_pt_coord));

                let col = &t_mat[row_idx];
                add_bottom((col, visited, basins, *low_pt_coord));
            } else {
                
            }
        }
    }

    return 1;
}

fn add_right(
    (
        row,
        visited,
        basins,
        low_pt_coord
    ):
    (
        &Vec<u32>,
        &mut HashSet<(usize, usize)>,
        &mut HashMap<(usize, usize), Vec<(usize, usize)>>,
        (usize, usize)
    )
) -> () {
    let r_idx = low_pt_coord.0;
    for (idx, point) in row.into_iter().enumerate() {
        if *point != 9 {
            visited.insert((r_idx, idx));
            basins.get_mut(&low_pt_coord).unwrap().push((r_idx, idx));
        }
    }
}

fn add_bottom(
    (
        col,
        visited,
        basins,
        low_pt_coord
    ):
    (
        &Vec<u32>,
        &mut HashSet<(usize, usize)>,
        &mut HashMap<(usize, usize), Vec<(usize, usize)>>,
        (usize, usize)
    )
) -> () {
    let c_idx = low_pt_coord.1;
    for (idx, point) in col.into_iter().enumerate() {
        if *point != 9 {
            visited.insert((idx, c_idx));
            basins.get_mut(&low_pt_coord).unwrap().push((idx, c_idx));
        }
    }
}

fn add_left(
    (
        row,
        visited,
        basins,
        low_pt_coord
    ):
    (
        &Vec<u32>,
        &mut HashSet<(usize, usize)>,
        &mut HashMap<(usize, usize), Vec<(usize, usize)>>,
        (usize, usize)
    )
) -> () {
    let r_idx = low_pt_coord.0;
    for (idx, point) in row.into_iter().enumerate().rev() {
        if *point != 9 {
            visited.insert((r_idx, idx));
            basins.get_mut(&low_pt_coord).unwrap().push((r_idx, idx));
        }
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn part_1(matrix: &Vec<Vec<u32>>) -> (u32, Vec<(usize, usize)>) {
    let mut low_pts: Vec<u32> = Vec::with_capacity(1000);
    let mut low_pts_coords: Vec<(usize, usize)> = Vec::with_capacity(1000);
    for (i, row) in matrix.iter().enumerate() {
        for (j, low_pt) in row.iter().enumerate() {
            // first row
            if i == 0 {
                if j == 0 {
                    // first elem
                    if low_pt < right(&row, j) &&
                        low_pt < down(&matrix, i, j) {
                        low_pts.push(*low_pt);
                        low_pts_coords.push((i, j));
                    }
                    // last elem
                } else if j == row.len() - 1 {
                    if low_pt < left(&row, j) &&
                        low_pt < down(&matrix, i, j) {
                        low_pts.push(*low_pt);
                        low_pts_coords.push((i, j));
                    }
                } else {
                    if low_pt < left(&row, j) &&
                        low_pt < right(&row, j) &&
                        low_pt < down(&matrix, i, j) {
                        low_pts.push(*low_pt);
                        low_pts_coords.push((i, j));
                    }
                }
            // last row
            } else if i == matrix.len() - 1 {
                if j == 0 {
                    // first elem
                    if low_pt < right(&row, j) &&
                        low_pt < up(&matrix, i, j) {
                        low_pts.push(*low_pt);
                        low_pts_coords.push((i, j));
                    }
                    // last elem
                } else if j == row.len() - 1 {
                    if low_pt < left(&row, j) &&
                        low_pt < up(&matrix, i, j) {
                        low_pts.push(*low_pt);
                        low_pts_coords.push((i, j));
                    }
                } else {
                    if low_pt < left(&row, j) &&
                        low_pt < right(&row, j) &&
                        low_pt < up(&matrix, i, j) {
                        low_pts.push(*low_pt);
                        low_pts_coords.push((i, j));
                    }
                }
            } else {
                if j == 0 {
                    // first elem
                    if low_pt < right(&row, j) &&
                        low_pt < down(&matrix, i, j) &&
                        low_pt < up(&matrix, i, j) {
                        low_pts.push(*low_pt);
                        low_pts_coords.push((i, j));
                    }
                    // last elem
                } else if j == row.len() - 1 {
                    if low_pt < left(&row, j) &&
                        low_pt < down(&matrix, i, j) &&
                        low_pt < up(&matrix, i, j) {
                        low_pts.push(*low_pt);
                        low_pts_coords.push((i, j));
                    }
                } else {
                    if low_pt < left(&row, j) &&
                        low_pt < right(&row, j) &&
                        low_pt < down(&matrix, i, j) && 
                        low_pt < up(&matrix, i, j){
                        low_pts.push(*low_pt);
                        low_pts_coords.push((i, j));
                    }
                }
            } 
        }  
    }

    let sum_risk_pts = low_pts
        .into_iter()
        .fold(0, |accum, height| accum + height + 1);
    return (sum_risk_pts, low_pts_coords);
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

