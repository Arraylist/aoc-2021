#![feature(int_abs_diff)]
use regex;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
    count: u32,
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}

fn main() {
    let mut vents: Vec<&str> = include_str!("./input.txt")
        .trim()
        .split("\n")
        .collect();

    let mut points: &mut Vec<Point> = &mut Vec::with_capacity(10000);

    for vent in vents {
        let re = regex::Regex::new(r"->").unwrap();
        let coords: Vec<&str> = re.split(vent).into_iter().collect();
        for (i, start) in re.split(vent).into_iter().enumerate().step_by(2) {
            if i + 1 > coords.len() - 1 {
                break;
            }
            let xy: Vec<u32> = to_cartesian(start);
            let (x1, y1): (u32, u32) = (xy[0], xy[1]);

            let xy_end: Vec<u32> = to_cartesian(coords[i + 1]);
            let (x2, y2): (u32, u32) = (xy_end[0], xy_end[1]);
            if x1 == x2 {
                let (start, end): (u32, u32) = get_range(y1, y2);
                update_floor((points, start, end, true, x1));
            } else if y1 == y2 {
                let (start, end): (u32, u32) = get_range(x1, x2);
                update_floor((points, start, end, false, y1));
            } else if is_diag((x1 as i32, y1 as i32, x2 as i32, y2 as i32)) {
                let diag_points = get_diag_points((x1 as i32, x2 as i32, y1 as i32, y2 as i32));
                for diag_point in diag_points {
                    let mut found: bool = false;
                    for point in points.iter_mut() {
                        if *point == diag_point {
                            found = true;
                            point.count += 1;
                            break;
                        }
                    }
                    if !found {
                        points.push(diag_point);
                    }
                }
            }
        } 
    }

    let danger_zone_cnt = points
        .into_iter()
        .fold(0, |accum, point| {
            if point.count >= 2 {
                return accum + 1;
            } else {
                return accum;
            }
        });

    println!("{:?}", danger_zone_cnt);
}

fn to_cartesian(str_coords: &str) -> Vec<u32> {
    return str_coords
            .trim()
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
}

fn get_range(start: u32, end: u32) -> (u32, u32) {
    if start > end {
       return (end, start);
    }
    return (start, end);
}

fn update_floor((points, start, end, is_y, orient): (&mut Vec<Point>, u32, u32, bool, u32)) -> () {
    for c in start..=end {
        let mut found: bool = false;
        let mut p: Point = Point { x: c, y: orient, count: 1 };
        if is_y {
            p = Point { x: orient, y: c, count: 1 };
        }
        for point in points.iter_mut() {
            if *point == p {
                found = true;
                point.count += 1;
                break;
            }
        }
        if !found {
            points.push(p);
        }
    }
}

fn is_diag((x1, y1, x2, y2): (i32, i32, i32, i32)) -> bool {
    return x1.abs_diff(x2) == y1.abs_diff(y2);
}

fn get_diag_points((x1, x2, y1, y2): (i32, i32, i32, i32)) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::with_capacity(200);
    let y_diff = y2 - y1;
    let x_diff = x2 - x1;
    let m: i32 = y_diff / x_diff;
    let c: i32 = y1 - m * x1;
    
    let (start, end) = get_range(x1 as u32, x2 as u32);

    for x in start..=end {
        points.push(Point { x: x as u32, y: (m * (x as i32) + c) as u32, count: 1 })
    }

    return points;
}
