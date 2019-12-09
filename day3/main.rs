use std::io::{self, Read};
use std::error::Error;

fn stdin_to_string() -> Result<String, Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    return Ok(input);
}

#[derive(Copy, Clone)]
struct Line {
    low_left: Point,
    up_right: Point,
    horizontal: bool,
}
impl Line{
    fn intersect(&self, other: &Line) -> Option<Point> {
        return None; // TODO
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

#[derive(Copy, Clone)]
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
    let wire1 = input_lines.next().unwrap().trim().split(',');
    let wire2: Vec<String> =
        input_lines.next().unwrap().trim().split(',').map(String::from).collect();

    let mut start1 = Point{x:0, y:0};
    let mut start2 = Point{x:0, y:0};
    let mut best_distance = std::i32::MAX;

    for seg1 in wire1 {
        let end1 = start1.offset(seg1);
        let line1 = make_line(start1, end1);
        start1 = end1;
        for seg2 in &wire2 {
            let end2 = start2.offset(&seg2);
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
    println!("Best intersection was at distance: {}", best_distance);
}
