/*Подключаем внешние библиотеки.
Также же в Cargo.toml

[dependencies]
rand="*"
three="*"
serde="*"
bincode="*"
serde_derive="*"

прописываем
*/
extern crate rand;
extern crate three;
extern crate bincode;
extern crate serde;
#[macro_use]
extern crate serde_derive;

// Добавляем нужные нам вещи в нашу область видимости.
use rand::Rng;
use three::*;
use std::error::Error;

//Entities ------------------------------------------------------------------

/*
Это макросы. Они генерируют какой нибудь код автоматически.
В нашем конкретном случае:
Debug - Создаст код который позволить выводить нашу структуру в терминал
Clone - Создаст код который будет копировать нашу структуру т. е. у нашей структуры появиться метод clone()
Eq и PartialEq позволять сравнивать наши Point с помошью оператора ==
*/
#[derive(Debug, Clone, Eq, PartialEq, Default)]
//Обьявление структуры с двумя полями. Она будет играть роль точки
struct Point {
    x: u8,
    y: u8,
}

//Методы нашей структуры
impl Point {
    // Можно было использовать просто оператор == В общем, это метот который проверяет пересекаються ли наши точки
    pub fn intersects(&self, point: &Point) -> bool {
        self.x == point.x && self.y == point.y
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
//Эта структура будет хранить обьектное представление границ фрейма в пределах которого будет двигаться наша змейка
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
//Обьявление перечисления с 4 вариантами
//Оно будет отвечать за то куда в данный момент повернута голова змейки
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

//Резализация трейта (в других языках это еще называеться интерфейс)
// для нашего перечесления.
//Обьект реализующий этот трейт способен иметь значение по умолчанию.
impl Default for Direction {
    fn default() -> Direction {
        return Direction::Right;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
//Собственно наша змейка
struct Snake {
    direction: Direction,
    points: std::collections::VecDeque<Point>,
    start_x: u8,
    start_y: u8,
}

impl Snake {
    //Статический метод конструктор для инициализации нового экземлпяра нашей змейки
    pub fn new(x: u8, y: u8) -> Snake {
        let mut points = std::collections::VecDeque::new();
        for i in 0..3 {
            points.push_front(Point { x: x + i, y: i + y });
        }
        Snake { direction: Direction::default(), points, start_x: x, start_y: y }
    }
    //Увеличивает длину нашей змейки на одну точку
    pub fn grow(mut self) -> Snake {
        if let Some(tail) = self.points.pop_back() {
            self.points.push_back(Point { x: tail.x, y: tail.y });
            self.points.push_back(tail);
        }
        self
    }

    //Сбрасывает нашу змейку в начальное состояние
    pub fn reset(self) -> Snake {
        Snake::new(self.start_x, self.start_y)
    }

    //Поворачивает голову змейки в нужном нам направлении
    pub fn turn(mut self, direction: Direction) -> Snake {
        self.direction = direction;
        self
    }

    //Если голова змейки достает до еды то увеличивает длину змейки на один и возврашает информацию о том была ли еда съедена
    pub fn try_eat(mut self, point: &Point) -> (Snake, bool) {
        let head = self.head();
        if head.intersects(point) {
            return (self.grow(), true);
        }
        (self, false)
    }

    //Если голова змейки столкнулась с фреймом то возвращает змейку в начальное состояние
    pub fn try_intersect_frame(mut self, frame: &Frame) -> Snake {
        let head = self.head();
        if frame.intersects(&head) {
            return self.reset();
        }
        self
    }

    //Если голова змейки столкнулась с остальной частью то возвращает змейку в начальное состояние.
    pub fn try_intersect_tail(mut self) -> Snake {
        let head = self.head();
        let p = self.points.clone();
        let points = p.into_iter().filter(|p| head.intersects(p));
        if points.count() > 1 {
            return self.reset();
        }
        self
    }

    //Дает голову змейки
    pub fn head(&self) -> Point {
        self.points.front().unwrap().clone()
    }

    //Перемещает змейку на одну точку в том направление куда в данный момент смотрит голова змейки
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
//Структура для создания новой еды для змейки
struct FoodGenerator {
    frame: Frame
}

impl FoodGenerator {
    //Создает новую точку в случайном месте в пределах фрейма
    pub fn generate(&self) -> Point {
        let x = rand::thread_rng().gen_range(self.frame.min_x + 1, self.frame.max_x);
        let y = rand::thread_rng().gen_range(self.frame.min_y + 1, self.frame.max_y);
        Point { x, y }
    }
}

#[derive(Serialize, Deserialize)]
//Хранит текущий и максимальный счет игры
struct ScoreRepository {
    score: usize
}

impl ScoreRepository {
    //Статический метод для сохранения текущего счета в файле
    // Result это перечесление которое может хранить в себе либо ошибку либо результат вычислений
    fn save(value: usize) -> Result<(), Box<Error>> {
        use std::fs::File;
        use std::io::Write;
        let score = ScoreRepository { score: value };
        //Сериализуем структуру в массив байтов с помощью библиотеки bincode
        let bytes: Vec<u8> = bincode::serialize(&score)?;
        //Создаем новый файл или если он уже сушествует то перезаписываем его.
        let mut file = File::create(".\\score.data")?;
        match file.write_all(&bytes) {
            Ok(t) => Ok(t),
            //Error это трейт а у трейт нет точного размера во время компиляции поэтому
            // нам надо обернуть значнеие в Box и в результате мы работает с указателем на
            //кучу в памяти где лежит наш обьект а не с самим обьектом а у указателя есть определенный размер
            // известный во время компиляции
            Err(e) => Err(Box::new(e))
        }
    }

    //Загружаем сохраненный результат из файла
    fn load() -> Result<usize, Box<Error>> {
        use std::fs::File;
        let mut file = File::open("./score.data")?;
        let data: ScoreRepository = bincode::deserialize_from(file)?;
        Ok(data.score)
    }
}

//Business Logic Layer------------------------------------------------------------

#[derive(Debug, Clone, Default)]
//Обьектное представление логики нашей игры
struct Game {
    snake: Snake,
    frame: Frame,
    food: Point,
    food_generator: FoodGenerator,
    score: usize,
    max_score: usize,
    total_time: f32,
}

impl Game {
    //Конструктор для создания игры с фреймом заданной высоты и ширины
    fn new(height: u8, width: u8) -> Game {
        let frame = Frame { min_x: 0, min_y: 0, max_x: width, max_y: height };
        let generator = FoodGenerator { frame: frame.clone() };
        let food = generator.generate();
        let snake = Snake::new(width / 2, height / 2);
        Game {
            snake,
            frame,
            food,
            food_generator: generator,
            score: 0,
            max_score: match ScoreRepository::load() {
                Ok(v) => v,
                Err(_) => 0
            },
            total_time: 0f32,
        }
    }
    // Проверяем, прошло ли достаточно времени с момента когда мы в последний раз
    //двигали нашу змейку и если да то передвигаем ее
    // и проверяем столкновение головы змейки с остальными обьектами игры
    fn update(mut self, time_delta_in_seconds: f32) -> Game {
        let (game, is_moving) = self.is_time_to_move(time_delta_in_seconds);
        self = game;
        if is_moving {
            self.snake = self.snake.clone()
                .move_snake()
                .try_intersect_tail()
                .try_intersect_frame(&self.frame);
            self.try_eat()
        } else {
            self
        }
    }

    //
    fn is_time_to_move(mut self, time_delta_in_seconds: f32) -> (Game, bool) {
        let time_to_move: f32 = 0.030;
        self.total_time += time_delta_in_seconds;
        if self.total_time > time_to_move {
            self.total_time -= time_to_move;
            (self, true)
        } else { (self, false) }
    }

    //Проверяем, сьела ли наша змейку еду и если да
    // то создаем новую еду, начисляем игроку очки
    // иначе сбрасываем игроку текуший счет
    fn try_eat(mut self) -> Game {
        let initial_snake_len = 3;
        if self.snake.points.len() == initial_snake_len {
            self.score = 0
        }
        let (snake, eaten) = self.snake.clone().try_eat(&self.food);
        self.snake = snake;
        if eaten {
            self.food = self.food_generator.generate();
            self.score += 1;
            if self.max_score < self.score {
                self.max_score = self.score;
                ScoreRepository::save(self.max_score);
            }
        };
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
#[derive(Debug, Clone, Default)]
struct GameController {
    game: Game,
}

impl GameController {
    fn new() -> GameController {
        GameController { game: Game::new(30, 30) }
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

    pub fn get_max_score(&self) -> usize {
        self.game.max_score.clone()
    }

    pub fn get_score(&self) -> usize {
        self.game.score.clone()
    }
}

//------------------------View ---------------
struct GameView {
    controller: GameController,
    window: three::Window,
    camera: three::camera::Camera,
    ambient: three::light::Ambient,
    directional: three::light::Directional,
    font: Font,
    current_score: Text,
    max_score: Text,

}

impl GameView {
    fn new() -> GameView {
        let controller = GameController::new();

        let mut window = three::Window::new("3D Snake Game By Victorem");

        let camera = window.factory.perspective_camera(60.0, 10.0..40.0);
        camera.set_position([15.0, 15.0, 30.0]);

        let ambient_light = window.factory.ambient_light(0xdc8874, 0.5);
        window.scene.add(&ambient_light);

        let mut dir_light = window.factory.directional_light(0xffffff, 0.9);
        dir_light.look_at([150.0, 350.0, 350.0], [0.0, 0.0, 0.0], None);
        let shadow_map = window.factory.shadow_map(2048, 2048);
        dir_light.set_shadow(shadow_map, 400.0, 1.0..1000.0);
        window.scene.add(&dir_light);
        let font = window.factory.load_font(format!("{}/DejaVuSans.ttf", env!("CARGO_MANIFEST_DIR")));
        let current_score = window.factory.ui_text(&font, "0");
        let mut max_score = window.factory.ui_text(&font, "0");
        max_score.set_pos([0.0, 40.0]);
        window.scene.add(&current_score);
        window.scene.add(&max_score);
        GameView { controller, window, camera, ambient: ambient_light, directional: dir_light, font, current_score, max_score }
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

    fn get_meshes(mut self) -> (Vec<Mesh>, GameView) {
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

        let meshes = self.controller.clone().get_state().iter().map(|s| {
            let state = s.clone();
            match state.state_type {
                PointDtoType::Frame => {
                    let m = self.window.factory.mesh(sphere.clone(), blue.clone());
                    m.set_position([state.x as f32, state.y as f32, 0.0]);
                    m
                }
                PointDtoType::Tail => {
                    let m = self.window.factory.mesh(sphere.clone(), yellow.clone());
                    m.set_position([state.x as f32, state.y as f32, 0.0]);
                    m
                }
                PointDtoType::Head => {
                    let m = self.window.factory.mesh(sphere.clone(), red.clone());
                    m.set_position([state.x as f32, state.y as f32, 0.0]);
                    m
                }
                PointDtoType::Food => {
                    let m = self.window.factory.mesh(sphere.clone(), green.clone());
                    m.set_position([state.x as f32, state.y as f32, 0.0]);
                    m
                }
            }
        }).collect();
        (meshes, self)
    }

    fn update(mut self) -> GameView {
        let elapsed_time = self.window.input.delta_time();
        let input = self.get_input();
        let controller = self.controller.update(elapsed_time, input);
        self.controller = controller;
        self
    }

    fn draw(mut self) -> GameView {
        let (meshes, view) = self.get_meshes();
        self = view;
        for m in &meshes {
            self.window.scene.add(m);
        }
        self.window.render(&self.camera);
        for m in meshes {
            self.window.scene.remove(m);
        }
        self.max_score.set_text(format!("MAX SCORE: {}", self.controller.get_max_score()));
        self.current_score.set_text(format!("CURRENT SCORE: {}", self.controller.get_score()));
        self
    }

    pub fn run(mut self) {
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
