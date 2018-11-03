extern crate rand;
extern crate three;

use rand::Rng;
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
#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct FoodGenerator {
    frame: Frame
}

impl FoodGenerator {
    pub fn generate(&self) -> Point {
        let x = rand::thread_rng().gen_range(self.frame.min_x + 1, self.frame.max_x);
        let y = rand::thread_rng().gen_range(self.frame.min_y + 1, self.frame.max_y);
        Point { x, y }
    }
}

//Business Logic Layer------------------------------------------------------------
#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct Game {
    snake: Snake,
    frame: Frame,
    food: Point,
    food_generator: FoodGenerator,
}

impl Game {
    fn new(height: u8, width: u8) -> Game {
        let frame = Frame { min_x: 0, min_y: 0, max_x: width, max_y: height };
        let generator = FoodGenerator { frame: frame.clone() };
        let food = generator.generate();
        let snake = Snake::new(width / 2, height / 2);
        Game {
            snake: snake,
            frame: frame,
            food: food,
            food_generator: generator,
        }
    }

    fn update(mut self, time_delta: f32) -> Game {
        let delta = time_delta as i32;
        if delta % 2 == 0 {
            self.snake = self.snake
                .move_snake()
                .try_intersect_tali()
                .try_intersect_frame(&self.frame)
                .try_eat(&self.food);
        }
        self
    }

    fn handle_input(mut self, input: Direction) -> Game {
        let snake = self.snake.turn(input);
        self.snake = snake;
        self
    }
}

//Application Layer--------------------------------------------------------------
// --- Model ----
#[derive(Debug, Clone, Eq, PartialEq)]
enum PointDtoType {
    Head,
    Tail,
    Food,
    Frame,
}
impl Default for PointDtoType {
    fn default() -> PointDtoType {
        PointDtoType::Frame
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct PointDto {
    x: u8,
    y: u8,
    state_type: PointDtoType,
}

//------------------------------Controller -----------------------------
#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct GameController {
    game: Game
}

impl GameController {
    fn new() -> GameController {
        GameController{game:Game::new(30,30)}
    }

    fn get_state(&self) -> Vec<PointDto> {
        let mut vec: Vec<PointDto> = Vec::new();
        vec.push(PointDto { x: self.game.food.x, y: self.game.food.y, state_type: PointDtoType::Food });
        let head = self.game.snake.head();
        vec.push(PointDto { x: head.x, y: head.y, state_type: PointDtoType::Head });
        for p in self.game.snake.points.iter().filter(|p| **p != head) {
            vec.push(PointDto { x: p.x, y: p.y, state_type: PointDtoType::Tail });
        }
        for x in self.game.frame.min_x..=self.game.frame.max_x {
            vec.push(PointDto { x: x, y: self.game.frame.max_y, state_type: PointDtoType::Frame });
            vec.push(PointDto { x: x, y: self.game.frame.min_y, state_type: PointDtoType::Frame });
        }
        for y in self.game.frame.min_y..=self.game.frame.max_y {
            vec.push(PointDto { x: self.game.frame.max_x, y: y, state_type: PointDtoType::Frame });
            vec.push(PointDto { x: self.game.frame.min_x, y: y, state_type: PointDtoType::Frame });
        }
        vec
    }

    fn update(mut self, time_delta: f32, direction: Option<Direction>) -> GameController {
        let game = self.game.clone();
        let game = match direction {
            None => game,
            Some(d) => game.handle_input(d)
        };
      let game = game.update(time_delta);
        self.game = game;
        self
    }
}

//------------------------View ---------------
struct GameView {
    controller: GameController,
    window: three::Window,
    camera: three::camera::Camera,
    timer: three::Timer,
    ambient: three::light::Ambient,
    directional: three::light::Directional,
}

impl GameView {
    fn new() -> GameView {
        let controller = GameController::new();

        let mut window = three::Window::new("3D Snake Game By Victorem");
        window.scene.background = three::Background::Color(0xf0e0b6);

        let camera = window.factory.perspective_camera(60.0, 10.0..40.0);
        camera.set_position([15.0, 15.0, 30.0]);

        let ambient_light = window.factory.ambient_light(0xdc8874, 0.5);
        window.scene.add(&ambient_light);

        let mut dir_light = window.factory.directional_light(0xffffff, 0.9);
        dir_light.look_at([150.0, 350.0, 350.0], [0.0, 0.0, 0.0], None);
        let shadow_map = window.factory.shadow_map(2048, 2048);
        dir_light.set_shadow(shadow_map, 400.0, 1.0..1000.0);
        window.scene.add(&dir_light);

        let mut timer = three::Timer::new();

        GameView{controller,window,camera,timer,ambient:ambient_light,directional:dir_light}
    }

    fn get_input(&self) -> Option<Direction> {
        match self.window.input.keys_hit().last() {
            None => None,
            Some(k) =>
                match *k {
                    three::Key::Left => Some(Direction::Left),
                    three::Key::Right => Some(Direction::Right),
                    three::Key::Down => Some(Direction::Top),
                    three::Key::Up => Some(Direction::Bottom),
                    _ => None,
                }
        }
    }

    fn get_meshes(&mut self)->Vec<Mesh>{
        let sphere = &three::Geometry::uv_sphere(0.5, 24, 24);
        let green = &three::material::Phong {
            color: three::color::GREEN,
            glossiness: 30.0,
        };
        let blue = &three::material::Phong {
            color: three::color::BLUE,
            glossiness: 30.0,
        };
        let red = &three::material::Phong {
            color: three::color::RED,
            glossiness: 30.0,
        };
        let yellow = &three::material::Phong {
            color: three::color::RED | three::color::GREEN,
            glossiness: 30.0,
        };

        self.controller.get_state().iter().map(|s| {
            let state = s.clone();
            match state.state_type {
                PointDtoType::Frame =>{
                    let m = self.window.factory.mesh(sphere.clone(),blue.clone());
                    m.set_position([state.x as f32,state.y as f32,0.0]);
                    m
                },
                PointDtoType::Tail =>{
                    let m= self.window.factory.mesh(sphere.clone(),yellow.clone());
                    m.set_position([state.x as f32,state.y as f32,0.0]);
                    m
                },
                PointDtoType::Head => {
                    let m = self.window.factory.mesh(sphere.clone(),red.clone());
                    m.set_position([state.x as f32,state.y as f32,0.0]);
                    m
                },
                PointDtoType::Food =>{
                    let m = self.window.factory.mesh(sphere.clone(),green.clone());
                    m.set_position([state.x as f32,state.y as f32,0.0]);
                    m
                }
            }
        }).collect()
    }

    fn update(mut self)-> GameView {
           let elapsed_time = self.timer.elapsed();
        let input = self.get_input();
       let controller = self.controller.update(elapsed_time,input);
        self.controller = controller;
        self
    }

    fn draw(mut self) -> GameView {
        let  meshes = self.get_meshes();
        for m in &meshes {
            self.window.scene.add(m);
        }
        self.window.render(&self.camera);
        for m in meshes {
            self.window.scene.remove(m);
        }
        self
    }

    pub fn run(mut self){
        while self.window.update() && !self.window.input.hit(three::KEY_ESCAPE) {
            self = self.update().draw();
        }
    }
}

fn main() {
    let mut view = GameView::new();
    view.run();
}

#[cfg(test)]
mod tests;
