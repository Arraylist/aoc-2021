#![feature(drain_filter)]

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
    println!("{}", part_2(&matrix, matrix[0].len()));
}

fn part_1(matrix: &Vec<Vec<u32>>, rate_len: usize) -> i32 {
    let transpose_matrix: Vec<Vec<u32>> = transpose(matrix.to_vec());
    let mut res = get_gamma_eps(transpose_matrix, rate_len);
    let gamma = res.remove(0);
    let epsilon = res.remove(0);
    let gamma_dec = isize::from_str_radix(&to_string(gamma), 2).unwrap();
    let epsilon_dec = isize::from_str_radix(&to_string(epsilon), 2).unwrap();

    return (gamma_dec * epsilon_dec) as i32;
}

fn part_2(matrix: &Vec<Vec<u32>>, rate_len: usize) -> i32 {
    let mut oxy: Vec<Vec<u32>> = filter(matrix.to_vec(), rate_len, false);
    let mut co2: Vec<Vec<u32>> = filter(matrix.to_vec(), rate_len, true);
    let oxy_dec = isize::from_str_radix(&to_string(oxy.remove(0)), 2).unwrap();
    let co2_dec = isize::from_str_radix(&to_string(co2.remove(0)), 2).unwrap();

    return (oxy_dec * co2_dec) as i32; 
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

fn get_gamma_eps(mat: Vec<Vec<u32>>, rate_len: usize) -> Vec<Vec<u32>> {
    let mut gamma: Vec<u32> = Vec::with_capacity(rate_len);
    let mut epsilon: Vec<u32> = Vec::with_capacity(rate_len);
    let mut res: Vec<Vec<u32>> = Vec::with_capacity(2);
    for row in mat {
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

    res.push(gamma);
    res.push(epsilon);
    return res;
}

fn filter(v: Vec<Vec<u32>>, u: usize, inv: bool) -> Vec<Vec<u32>> {
    let mut res: Vec<Vec<u32>> = v.clone();
    for i in 0..u {
        if res.len() == 1 {
            break;
        }
        let mut ones = 0;
        let mut zeroes = 0;
        for v in &res {
            if v[i] == 0 {
                zeroes += 1;
            } else if v[i] == 1 {
                ones += 1;
            }
        }
        res = res.drain_filter(|v| {
            if ones == zeroes || ones > zeroes {
                if inv {
                    return v[i] == 0;
                }
                return v[i] == 1;
            }
            if inv {
                return v[i] == 1;
            }
            return v[i] == 0;
        }).collect();
    }
    return res;
}

fn to_string(v: Vec<u32>) -> String {
    let mut v_str = String::new();
    for num in v {
        v_str.push_str(&num.to_string())
    }
    return v_str;
}
