fn main() {
    let diag_report: Vec<&str> = include_str!("./input.txt")
        .trim()
        .split("\n")
        .collect();
    let mut matrix: Vec<Vec<u32>> = Vec::with_capacity(diag_report.len());

    for line in diag_report {
        let numbers: Vec<u32> = line
            .chars()
            .map(|d| d.to_digit(2).unwrap())
            .collect();
        
        matrix.push(numbers);
    }
    println!("{}", part_1(&matrix, matrix[0].len()));
}

fn part_1(matrix: &Vec<Vec<u32>>, rate_len: usize) -> i32 {
    let transpose_matrix: Vec<Vec<u32>> = transpose(matrix.to_vec());
    let mut gamma: Vec<u32> = Vec::with_capacity(rate_len);
    let mut epsilon: Vec<u32> = Vec::with_capacity(rate_len);
    for row in transpose_matrix {
        let mut zeroes = 0;
        let mut ones = 0;
        for number in row {
            if number == 0 {
                zeroes += 1;
            } else if number == 1 {
                ones += 1;
            }
        }
        match ones > zeroes {
            false => {
                gamma.push(0);
                epsilon.push(1);
            },
            true => {
                gamma.push(1);
                epsilon.push(0);
            },
        };
    }
    let gamma_dec = isize::from_str_radix(&to_string(gamma), 2).unwrap();
    let epsilon_dec = isize::from_str_radix(&to_string(epsilon), 2).unwrap();

    return (gamma_dec * epsilon_dec) as i32;
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

fn to_string(v: Vec<u32>) -> String {
    let mut v_str = String::new();
    for num in v {
        v_str.push_str(&num.to_string())
    }
    return v_str;
}

