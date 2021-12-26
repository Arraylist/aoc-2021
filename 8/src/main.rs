#![feature(string_remove_matches)]
use std::collections::HashSet;

fn main() {
    let mut patterns_outputs: Vec<(&str, &str)> = include_str!("./input.txt")
        .trim()
        .lines()
        .filter_map(|line| line.split_once(" | "))
        .collect();

    println!("{}", part_2(&patterns_outputs));

    // let mut outputs: Vec<&str> = Vec::with_capacity(1000);
    // for po in patterns_outputs {
    //     outputs.push(po.0);
    // }
    
    // println!("{}", part_1(&outputs));
}

fn part_1(outputs: &Vec<&str>) -> usize {
    let mut single_digit_outputs: Vec<&str> = Vec::with_capacity(1000);
    outputs.into_iter()
        .for_each(|s| {
            let output: Vec<&str> = s.split_whitespace().collect();
            for o in output {
                single_digit_outputs.push(o);
            }
        });

    return single_digit_outputs
        .into_iter()
        .fold(0, |accum, digit| {
            match digit.len() {
                2 | 3 | 4 | 7 => accum + 1,
                _ => accum,
            }
        });
}

fn part_2(patterns_outputs: &Vec<(&str, &str)>) -> usize {
    let mut output_values: Vec<usize> = Vec::with_capacity(1000);
    patterns_outputs.into_iter()
        .for_each(|(pattern, output)| {
            let mut decoded_digits: Vec<String> = vec!['a'.to_string(); 10];
            let mut p: Vec<&str> = pattern.split_whitespace().collect();
            let mut zero_six_nine: Vec<String> = Vec::with_capacity(3);
            let mut two_three_five: Vec<String> = Vec::with_capacity(3);
            for encoded_digit in p {
                match encoded_digit.len() {
                    2 => decoded_digits[1] = sort_chars(encoded_digit),
                    3 => decoded_digits[7] = sort_chars(encoded_digit),
                    4 => decoded_digits[4] = sort_chars(encoded_digit),
                    5 => two_three_five.push(sort_chars(encoded_digit)),
                    6 => zero_six_nine.push(sort_chars(encoded_digit)),
                    7 => decoded_digits[8] = sort_chars(encoded_digit),
                    _ => (),
                }
            }
            let s1 = String::from(&decoded_digits[7]);
            let s2 = String::from(&decoded_digits[1]);
            let top = get_odd_char(&s1, &s2);
            let bl = get_bottom_left(&zero_six_nine, &decoded_digits[4], &top);
            let bottom = bl.0;
            let bottom_left = bl.1;
            let m_tr_br_tl = get_rest(&two_three_five, &top, &bottom, &bottom_left);
            let middle = m_tr_br_tl.0;
            let top_right = m_tr_br_tl.1;
            let bottom_right = m_tr_br_tl.2;
            let top_left = m_tr_br_tl.3;

            let mut zero: String = top.to_owned();
            zero.push_str(&bottom);
            zero.push_str(&bottom_left);
            zero.push_str(&top_left);
            zero.push_str(&top_right);
            zero.push_str(&bottom_right);
            decoded_digits[0] = sort_chars(&zero);

            let mut two: String = top.to_owned();
            two.push_str(&top_right);
            two.push_str(&middle);
            two.push_str(&bottom_left);
            two.push_str(&bottom);
            decoded_digits[2] = sort_chars(&two);

            let mut three: String = top.to_owned();
            three.push_str(&top_right);
            three.push_str(&middle);
            three.push_str(&bottom_right);
            three.push_str(&bottom);
            decoded_digits[3] = sort_chars(&three);

            let mut five: String = top.to_owned();
            five.push_str(&top_left);
            five.push_str(&middle);
            five.push_str(&bottom_right);
            five.push_str(&bottom);
            decoded_digits[5] = sort_chars(&five);

            let mut six: String = top.to_owned();
            six.push_str(&bottom);
            six.push_str(&bottom_left);
            six.push_str(&top_left);
            six.push_str(&middle);
            six.push_str(&bottom_right);
            decoded_digits[6] = sort_chars(&six);

            let mut nine: String = top.to_owned();
            nine.push_str(&bottom);
            nine.push_str(&top_right);
            nine.push_str(&top_left);
            nine.push_str(&middle);
            nine.push_str(&bottom_right);
            decoded_digits[9] = sort_chars(&nine);

            let outs: Vec<&str> = output.split_whitespace().collect();
            let mut val: String = "".to_string();
            for o in outs {
                let so = sort_chars(o);
                for (i, digit) in decoded_digits.iter().enumerate() {
                    if so == *digit {
                        val.push_str(&i.to_string());
                    }
                }
            }

            let num_val = val.parse::<usize>().unwrap();
            output_values.push(num_val);
        });

    return output_values
        .into_iter()
        .fold(0, |accum, output_value| accum + output_value);
}

fn get_rest(
    two_three_five: &Vec<String>,
    top: &String,
    bottom: &String,
    bottom_left: &String
) -> (String, String, String, String) {
    let numbers = two_three_five.clone(); 
    let mut tbl_str: String = top.to_owned();
    tbl_str.push_str(bottom);
    tbl_str.push_str(bottom_left);
    let set: HashSet<char> = tbl_str.chars().collect();

    let commons: Vec<Vec<char>> = numbers
        .into_iter()
        .map(|number| number.chars().filter(|c| !set.contains(&c)).collect())
        .collect();

    let a: HashSet<char> = commons[0].iter().cloned().collect();
    let b: HashSet<char> = commons[1].iter().cloned().collect();
    let c: HashSet<char> = commons[2].iter().cloned().collect();
    let mut middle = " ".to_string();
    let mut sets: Vec<HashSet<char>> = Vec::new();
    sets.push(a);
    sets.push(b);
    sets.push(c);

    let intersection = sets
        .iter()
        .skip(1)
        .fold(sets[0].clone(), |acc, hs| {
            acc.intersection(hs).cloned().collect()
        });

    for i in intersection {
        middle = i.to_string();
    }


    let mut two: Vec<char> = Vec::with_capacity(2);
    let mut three_five: Vec<Vec<char>> = Vec::with_capacity(2);
    commons.into_iter().for_each(|n: Vec<char>| {
        if n.len() == 2 {
            two = n;
        } else {
            three_five.push(n);
        }
    });
    let mut two_str: String = two.into_iter().collect();
    let top_right: String = get_odd_char(&two_str, &middle);

    let mut three_five_0: String = " ".to_string();
    let mut three_five_1: String = " ".to_string();

    for (i, tf) in three_five.iter().enumerate() {
        if i == 0 {
            three_five_0 = tf.into_iter().collect();
        } else if i == 1 {
            three_five_1 = tf.into_iter().collect();
        }
    }

    let odd1 = get_common_chars(&two_str, &three_five_0);
    let odd2 = get_common_chars(&two_str, &three_five_1);
    let mut bottom_right = " ".to_string();
    if odd2.len() > odd1.len() {
        bottom_right = get_odd_char(&three_five_1, &two_str);
    } else {
        bottom_right = get_odd_char(&three_five_0, &two_str);
    }
    let all: String = "abcdefg".to_string();
    let mut almost_all: String = top.to_owned();
    almost_all.push_str(bottom);
    almost_all.push_str(bottom_left);
    almost_all.push_str(&middle);
    almost_all.push_str(&top_right);
    almost_all.push_str(&bottom_right);
    let top_left: String = get_odd_char(&all, &almost_all);


    return (middle, top_right, bottom_right, top_left);
}

fn get_bottom_left(zero_six_nine: &Vec<String>, four: &String, top: &String) -> (String, String) {
    let numbers = zero_six_nine.clone(); 
    let set: HashSet<char> = four.chars().collect();
    let mut nine: Vec<char> = Vec::with_capacity(2);
    let mut zero_six: Vec<char> = Vec::with_capacity(3);

    let mut commons: Vec<Vec<char>> = numbers
        .into_iter()
        .map(|number| number.chars().filter(|c| !set.contains(&c)).collect())
        .collect();

        commons.into_iter().for_each(|n: Vec<char>| {
            if n.len() == 2 {
                nine = n;
            } else {
                zero_six = n;
            }
        });
    
    let nine_string: String = nine.into_iter().collect();
    let bottom_left: String = get_odd_char(&nine_string, &zero_six.into_iter().collect());
    let bottom: String = get_odd_char(&nine_string, top);

    return (bottom, bottom_left);
}

fn get_odd_char(a: &String, b: &String) -> String {
    let (shorter, longer) = if a.len() > b.len() {
        (b, a)
    }  else {
        (a, b)
    };

    let set: HashSet<char> = shorter.chars().collect();

    return longer.chars().find(|c| !set.contains(&c)).unwrap().to_string();
}

fn get_common_chars(a: &String, b: &String) -> String {
    let (shorter, longer) = if a.len() > b.len() {
        (b, a)
    }  else {
        (a, b)
    };

    let short_set: HashSet<char> = shorter.chars().collect();
    let long_set: HashSet<char> = longer.chars().collect();

    return long_set.intersection(&short_set).collect();
}

fn sort_chars(unsorted: &str) -> String {
    let mut v1: Vec<char> = unsorted.chars().collect();
    v1.sort_by(|a, b| a.cmp(b));
    return v1.iter().collect::<String>();
}
