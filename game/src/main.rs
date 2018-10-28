

use termion::*;
//Entities ------------------------------------------------------------------

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    pub fn intersects(&self, point: &Point) -> bool {
        self.x == point.x && self.y == point.y
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct Frame {
    min_x: u8,
    min_y: u8,
    max_x: u8,
    max_y: u8,
}

impl Frame {
    pub fn intersects(&self, point: &Point) -> bool {
        point.x == self.min_x
            || point.y == self.min_y
            || point.x == self.max_x
            || point.y == self.max_y
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

impl Default for Direction {
    fn default() -> Direction {
        return Direction::Right;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct Snake {
    direction: Direction,
    points: std::collections::VecDeque<Point>,
    start_x: u8,
    start_y: u8,
}

impl Snake {
    pub fn new(x: u8, y: u8) -> Snake {
        let mut points = std::collections::VecDeque::new();
        for i in 0..3 {
            points.push_front(Point { x: x + i, y: i + y });
        }
        Snake { direction: Direction::default(), points, start_x: x, start_y: y }
    }

    pub fn grow(mut self) -> Snake {
        if let Some(tail) = self.points.pop_back() {
            self.points.push_back(Point { x: tail.x, y: tail.y });
            self.points.push_back(tail);
        }
        self
    }

    pub fn reset(self) -> Snake {
        Snake::new(self.start_x, self.start_y)
    }

    pub fn turn(mut self, direction: Direction) -> Snake {
        self.direction = direction;
        self
    }

    pub fn try_eat(mut self, point: &Point) -> Snake {
        let head = self.head();
        if head.intersects(point) {
            return self.grow();
        }
        self
    }

    pub fn try_intersect_frame(mut self, frame: &Frame) -> Snake {
        let head = self.head();
        if frame.intersects(&head) {
            return self.reset();
        }
        self
    }

    pub fn try_intersect_tali(mut self) -> Snake {
        let head = self.head();
        let p = self.points.clone();
        let points = p.into_iter().filter(|p| head.intersects(p));
        if points.count() > 1 {
            return self.reset();
        }
        self
    }

    pub fn head(&self) -> Point {
        self.points.front().unwrap().clone()
    }

    pub fn move_snake(mut self) -> Snake {
        if let Some(mut tail) = self.points.pop_back() {
            let head = self.head();
            match self.direction {
                Direction::Right => {
                    tail.x = head.x + 1;
                    tail.y = head.y;
                }
                Direction::Left => {
                    tail.x = head.x - 1;
                    tail.y = head.y;
                }
                Direction::Top => {
                    tail.x = head.x;
                    tail.y = head.y - 1;
                }
                Direction::Bottom => {
                    tail.x = head.x;
                    tail.y = head.y + 1;
                }
            }
            self.points.push_front(tail);
        }
        self
    }
}

//Data Access Layer ----------------------------------------------------------------

struct PointWriter {
    symbol: char
}

impl PointWriter {
    pub fn new(symbol: char) -> PointWriter {
        PointWriter { symbol }
    }
    pub fn write(&self, point: &Point) {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}", termion::cursor::Goto(point.x.into(), point.y.into()), self.symbol)
            .unwrap();
        stdout.flush().unwrap();
    }
}

struct FrameWriter {
    symbol: char
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests;
