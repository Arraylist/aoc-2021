fn main() {
    let mut bingo: Vec<&str> = include_str!("./input.txt")
        .trim()
        .split("\n")
        .collect();

    let numbers: Vec<u32> = bingo.remove(0)
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    bingo.remove(0);

    let mut boards: Vec<Vec<Vec<u32>>> = Vec::with_capacity(bingo.len());
    let mut board: Vec<Vec<u32>> = Vec::with_capacity(5);

    let mut mark_boards: Vec<Vec<Vec<bool>>> = Vec::with_capacity(bingo.len());
    let mut mark_board: Vec<Vec<bool>> = Vec::with_capacity(5);

    for row in bingo {
        if row.len() > 0 {
            let valid_row: Vec<u32> = row.split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>().unwrap())
                .collect();

            let marks: Vec<bool> = row.split(' ')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| false)
            .collect();

            board.push(valid_row);
            mark_board.push(marks);
        } else {
            boards.push(board);
            board = Vec::with_capacity(5);

            mark_boards.push(mark_board);
            mark_board = Vec::with_capacity(5);
        }
    }

    if board[0].len() > 0 {
        boards.push(board);
    }

    if mark_board[0].len() > 0 {
        mark_boards.push(mark_board);
    }

    println!("{}", part_1(&boards, &mut mark_boards, numbers).0);
    println!("{}", part_2(&boards, &mut mark_boards, numbers));
}


fn part_1(boards: &Vec<Vec<Vec<u32>>>, mark_boards: &mut Vec<Vec<Vec<bool>>>, numbers: Vec<u32>) -> (u32, usize, usize) {
    let mut win_board: Vec<Vec<u32>> = vec![
        Vec::with_capacity(5),
        Vec::with_capacity(5),
        Vec::with_capacity(5),
        Vec::with_capacity(5),
        Vec::with_capacity(5)
    ];
    let mut mark_win_board: Vec<Vec<bool>> = vec![
        Vec::with_capacity(5),
        Vec::with_capacity(5),
        Vec::with_capacity(5),
        Vec::with_capacity(5),
        Vec::with_capacity(5)
    ];
    let mut bingo_nbr: u32 = 0;
    let mut sum_unmarked: u32 = 0;
    let mut near_finishes: Vec<(usize, usize, usize)> = Vec::with_capacity(5 * 5 * boards.len());
    let mut location: (usize, usize) = (0, 0);
    let mut win_board_idx: usize = 0;
    let mut bingo_idx: usize = 0;
    for (nn, n) in numbers.into_iter().enumerate() {
        if bingo_nbr > 0 {
            break;
        }
        for (i, board) in boards.into_iter().enumerate() {
            if bingo_nbr > 0 {
                break;
            }
            let trans_mark_board: Vec<Vec<bool>> = transpose(mark_boards[i].to_vec());
            for (j, row) in board.into_iter().enumerate() {
                let win_row: bool = mark_boards[i][j].iter()
                    .all(|&e| e == true);
                let win_col: bool = trans_mark_board[j].iter()
                    .all(|&e| e == true);
                if win_row || win_col {
                    win_board = board.to_vec();
                    mark_win_board = mark_boards[i].to_vec();
                    for near in &near_finishes {
                        if win_row {
                            if near.0 == i && near.1 == j {
                                location.0 = near.1;
                                location.1 = near.2;
                                break;
                            }
                        } else if win_col {
                            if near.0 == i && near.2 == j {
                                location.0 = near.1;
                                location.1 = near.2;
                                break;
                            }
                        }
                    }
                    bingo_nbr = win_board[location.0][location.1];
                    bingo_idx = nn;
                    win_board_idx = i;
                    break;
                }

                let mut near_finish_row_cnt = 0;
                let mut near_finish_col_cnt = 0;
                mark_boards[i][j].iter().for_each(|mark| {
                    if *mark {
                        near_finish_row_cnt += 1;
                    }
                });
                trans_mark_board[j].iter().for_each(|mark| {
                    if *mark {
                        near_finish_col_cnt += 1;
                    }
                });
                if near_finish_row_cnt == 4 {
                    for (ii, mark) in mark_boards[i][j].to_vec().into_iter().enumerate() {
                        if !mark {
                            near_finishes.push((i, j, ii));

                        }
                    }
                }
                if near_finish_col_cnt == 4 {
                    for (ii, mark) in trans_mark_board[j].to_vec().into_iter().enumerate() {
                        if !mark {
                            near_finishes.push((i, ii, j));
                        }
                    }
                }
                for (k, be) in row.into_iter().enumerate() {
                    if *be == n {
                        mark_boards[i][j][k] = true;
                    }
                }
            }
        }
    }

    for (i, row) in win_board.into_iter().enumerate() {
        for (j, number) in row.into_iter().enumerate() {
            if !mark_win_board[i][j] {
                sum_unmarked += number;
            }
        }
    }

    return (sum_unmarked * bingo_nbr, win_board_idx, bingo_idx);
}

fn part_2(boards: &Vec<Vec<Vec<u32>>>, mark_boards: &mut Vec<Vec<Vec<bool>>>, numbers: Vec<u32>) -> u32 {
    let mut boards_cpy = boards.clone();
    let mut numbers_cpy = numbers.clone();
    let mut mark_boards_cpy = mark_boards.clone();
    while boards_cpy.len() > 1 {
        let score_board_next = part_1(&boards_cpy, &mut mark_boards_cpy, numbers_cpy.to_vec());
        boards_cpy.remove(score_board_next.1);
        numbers_cpy = numbers_cpy.split_off(score_board_next.2);
        mark_boards_cpy.remove(score_board_next.1);
    }

    if boards_cpy.len() == 1 {
        let fnl = part_1(&boards_cpy, &mut mark_boards_cpy, numbers_cpy.to_vec());
        return fnl.0;
    }

    return 0;
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
