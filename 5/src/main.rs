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

    let mut points: Vec<Point> = Vec::with_capacity(10000);

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
                let mut start: u32 = y1;
                let mut end: u32 = y2;
                if y1 > y2 {
                    start = y2;
                    end = y1;
                }
                for y in start..=end {
                    let mut found: bool = false;
                    let p = Point { x: x1, y: y, count: 1 };
                    for point in &mut points {
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
            } else if y1 == y2 {
                let mut start: u32 = x1;
                let mut end: u32 = x2;
                if x1 > x2 {
                    start = x2;
                    end = x1;
                }
                for x in start..=end {
                    let mut found: bool = false;
                    let p = Point { x: x, y: y1, count: 1 };
                    for point in &mut points {
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
