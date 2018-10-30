extern crate rand;
extern crate three;

use rand::Rng;
//extern crate cgmath;

//use cgmath::prelude::*;
use three::*;

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

struct FoodGenerator {}

impl FoodGenerator {
    pub fn generate(in_frame: &Frame) -> Point {
        let x = rand::thread_rng().gen_range(in_frame.min_x + 1, in_frame.max_x);
        let y = rand::thread_rng().gen_range(in_frame.min_y + 1, in_frame.max_y);
        Point { x, y }
    }
}

//Business Logic Layer------------------------------------------------------------

enum StateType {
    Head,
    Tail,
    Food,
    Frame,
}

struct State {
    x: u8,
    y: u8,
    state_type: StateType,
}

struct Game {
    snake: Snake,
    frame: Frame,
    food: Point,
}

impl Game {
    fn update(mut self, direction: Option<Direction>, time_delta: f32) -> Game {
        if delta % 3 = 0 {
            let mut snake = self.snake.try_intersect_tali()
                .try_intersect_frame(&self.frame)
                .try_eat(&self.food);
            if let Some(d) = direction {
                snake = snake.turn(d);
            };
            let delta: isize = time_delta.into();

            snake = snake.move_snake();
            self.snake = snake;
        }
        self
    }

    fn draw(&self) -> Vec<State> {
        let mut vec: Vec<State> = Vec::new();
        vec.push(State { x: self.food.x, y: self.food.y, state_type: StateType::Food });
        let head = self.snake.head();
        vec.push(State { x: head.x, y: head.y, state_type: StateType::Head });
        for p in self.snake.points.iter().filter(|p| **p != head) {
            vec.push(State { x: p.x, y: p.y, state_type: StateType::Tail });
        }
        for x in self.frame.min_x..=self.frame.max_x {
            vec.push(State { x: x, y: self.frame.max_y, state_type: StateType::Frame });
            vec.push(State { x: x, y: self.frame.min_y, state_type: StateType::Frame });
        }
        for y in self.frame.min_y..=self.frame.max_y {
            vec.push(State { x: self.frame.max_x, y: y, state_type: StateType::Frame });
            vec.push(State { x: self.frame.min_x, y: y, state_type: StateType::Frame });
        }
        vec
    }
}

//Application Layer

struct GameController {}

impl GameController {
    fn get_state() -> Vec<Mesh> {
        Vec::new()
    }

    fn update(time_delta: f32, direction: Option<direction>) {}
}

fn main() {
    let mut window = three::Window::new("3D Snake Game By Victorem");
    window.scene.background = three::Background::Color(0xf0e0b6);

    let camera = window.factory.perspective_camera(75.0, 1.0..50.0);
    camera.set_position([3.0, 3.0, 40.0]);

    let sphere = three::Geometry::uv_sphere(5.0, 24, 24);
    let green = three::material::Phong {
        color: three::color::GREEN,
        glossiness: 30.0,
    };
    let blue = three::material::Basic {
        color: three::color::BLUE,
        map: None,
    };
    let red = three::material::Basic {
        color: three::color::RED,
        map: None,
    };
    let yellow = three::material::Basic {
        color: three::color::RED | three::color::GREEN,
        map: None,
    };

    let green_sphere = window.factory.mesh(sphere, green);
    let blue_sphere = window.factory.mesh_instance_with_material(&green_sphere, blue);
    let red_sphere = window.factory.mesh_instance_with_material(&green_sphere, red);
    let yellow_sphere = window.factory.mesh_instance_with_material(&green_sphere, yellow);
    green_sphere.set_position([0.0, 0.0, 0.0]);
    window.scene.add(&green_sphere);

    let ambient_light = window.factory.ambient_light(0xdc8874, 0.5);
    window.scene.add(&ambient_light);

    let mut dir_light = window.factory.directional_light(0xffffff, 0.9);

    dir_light.look_at([150.0, 350.0, 350.0], [0.0, 0.0, 0.0], None);

    let shadow_map = window.factory.shadow_map(2048, 2048);

    dir_light.set_shadow(shadow_map, 400.0, 1.0..1000.0);

    window.scene.add(&dir_light);


    let mut timer = three::Timer::new();

    while window.update() && !window.input.hit(three::KEY_ESCAPE) {
        println!("{}", timer.elapsed());
        window.render(&camera);
        if window.input.hit(three::Key::Left) {
            window.scene.background = three::Background::Color(0xFFFF00);
        } else if window.input.hit(three::Key::Right) {
            window.scene.background = three::Background::Color(0xFF0000);
        } else if window.input.hit(three::Key::Up) {
            window.scene.background = three::Background::Color(0x00FF00);
        } else if window.input.hit(three::Key::Down) {
            window.scene.background = three::Background::Color(0x0000FF);
        }
    }
}

#[cfg(test)]
mod tests;
