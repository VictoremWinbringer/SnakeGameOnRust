

//Entities

#[derive(Debug,Clone)]
struct Point {
    x:u8,
    y:u8
}

impl Point {
    pub fn intersects(&self, point:&Point) -> bool {
        self.x == point.x && self.y == point.y
    }
}

struct Frame {
    min_x: u8,
    min_y: u8,
    max_x: u8,
    max_y: u8
}

impl Frame {
    pub fn intersects(&self, point:&Point) -> bool {
        point.x == self.min_x
        || point.y == self.min_y
        || point.x == self.max_x
        || point.y == self.max_y
    }
}

enum Direction {
    Left,
    Right,
    Top,
    Bottom
}

struct Snake {
    direction: Direction,
    points: std::collections::VecDeque<Point>,
    start_x:u8,
    start_y:u8,
    isAlive: bool
}

impl Snake {
    pub fn new(x:u8, y:u8) -> Snake {
        let mut points = std::collections::VecDeque::new();
        for i in 0..3 {
            points.push_front(Point{x:x+i,y: i + y });
        }
        Snake{direction:Direction::Right, points:points, start_x:x,start_y:y, isAlive: true}
    }

    pub fn grow(mut self) -> Snake {
        if let Some(tail) = self.points.pop_back() {
            self.points.push_back(Point{x:tail.x,y:tail.y});
            self.points.push_back(tail);
        }
        self
    }

    pub fn reset(self) -> Snake{
         Snake::new(self.start_x,self.start_y)
    }

    pub fn turn(mut self, direction:Direction) -> Snake {
        self.direction = direction;
        self
    }

    pub fn try_eat(mut self, point:&Point) -> Snake {
            if let Some(head) = self.points.pop_front() {
                let h = head.clone();
                self.points.push_front(h);
                if head.intersects(point) {
                 return  self.grow();
                }
            }
        self
    }

    pub fn try_intersect_frame(mut self, frame:&Frame) -> Snake {
        if let Some(head) = self.points.pop_front() {
            let h = head.clone();
            self.points.push_front(h);
            if frame.intersects(&head) {
              return   self.reset()
            }
        }
        self
    }

    pub fn try_intersect_tali(mut self) -> Snake {
        if let Some(head) = self.points.pop_front() {
            let h = head.clone();
            self.points.push_front(h);
            let p = self.points.clone();
            let points = p.into_iter().filter(|p| head.intersects(p));
            if points.count() > 1 {
              return self.reset()
            }
        }
        self
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_intesect() {
        let p1 = Point{x:1,y:1};
        let p2 = Point{x:1,y:1};
        let p  = Point{x:2,y:1};
        assert_eq!(true,p2.intersects(&p1));
        assert_eq!(true,p1.intersects(&p2));
        assert_eq!(false,p1.intersects(&p));
        assert_eq!(false,p.intersects(&p1));
    }
}