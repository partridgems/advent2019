use std::io::{self, Read};
use std::error::Error;

fn stdin_to_string() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    return Ok(input);
}

#[derive(Copy, Clone, Debug)]
struct Line {
    low_left: Point,
    up_right: Point,
    horizontal: bool,
}
impl Line{
    fn intersect(&self, other: &Line) -> Option<Point> {
        if !(self.horizontal ^ other.horizontal) { return None; }
 
        let h_line = if self.horizontal { self } else { other };
        let v_line = if self.horizontal { other } else { self };
        if h_line.low_left.y > v_line.low_left.y &&
            h_line.low_left.y < v_line.up_right.y &&
            v_line.low_left.x > h_line.low_left.x &&
            v_line.low_left.x < h_line.up_right.x {
                return Some(Point{x:v_line.low_left.x, y:h_line.low_left.y});
            } else {
                return None;
            }
    }
}

fn make_line(start: Point, finish: Point) -> Line {
    let horizontal = start.y == finish.y;
    if start.x < finish.x {
        return Line{low_left: start, up_right: finish, horizontal};
    } else if start.x > finish.x {
        return Line{low_left: finish, up_right: start, horizontal};
    } else if start.y < finish.y {
        return Line{low_left: start, up_right: finish, horizontal};
    } else {
        return Line{low_left: finish, up_right: start, horizontal};
    }
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn offset(&self, travel: &str) -> Point {
        let dir = travel.chars().next().unwrap();
        let len: i32 = travel[1..].parse().unwrap();
        match dir {
            'U' => Point{x:self.x, y:self.y+len},
            'D' => Point{x:self.x, y:self.y-len},
            'L' => Point{x:self.x-len, y:self.y},
            'R' => Point{x:self.x+len, y:self.y},
            _   => panic!("Unknown segment description: {}", travel)
        }
    }
}

fn main() {
    let input = stdin_to_string().unwrap();
    let mut input_lines = input.lines();
    let wire1 = input_lines.next().unwrap().trim();
    let wire2 = input_lines.next().unwrap().trim();

    let mut best_distance = std::i32::MAX;

    let mut start1 = Point{x:0, y:0};
    for seg1 in wire1.split(',') {
        let end1 = start1.offset(seg1);
        let line1 = make_line(start1, end1);
        start1 = end1;

        let mut start2 = Point{x:0, y:0};
        for seg2 in wire2.split(',') {
            let end2 = start2.offset(seg2);
            let line2 = make_line(start2, end2);
            start2 = end2;
            // Check intersection, record best
            match line1.intersect(&line2) {
                Some(p) => {
                    if p.x == 0 && p.y == 0 {
                        continue;
                    }
                    else {
                        best_distance = std::cmp::min(best_distance, p.x.abs() + p.y.abs());
                    }
                }
                None => {
                    continue;
                }
            }
        }
    }
    if best_distance == std::i32::MAX { println!("ERROR: found no intersection."); }
    else { println!("Best intersection was at distance: {}", best_distance); }
}
