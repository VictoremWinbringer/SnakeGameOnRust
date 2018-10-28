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

#[test]
fn test_frame_intersects() {
    let f1 = Frame {
        min_x: 1,
        min_y: 1,
        max_x: 2,
        max_y: 2,
    };
    let p1 = Point{x:1,y:1};
    let p2 = Point{x:3,y:3};

    assert_eq!(true, f1.intersects(&p1));
    assert_eq!(false, f1.intersects(&p2));
}

#[test]
fn test_snake_new(){
    let snake = Snake::new(1,2);
    assert_eq!(1, snake.start_x);
    assert_eq!(2, snake.start_y);
    assert_eq!(Direction::Right, snake.direction);
}

#[test]
fn test_snake_grow(){
    let snake = Snake::new(1,2);
    let old = snake.points.clone();
    let newSnake = snake.grow();
    let new = newSnake.points.clone();
    assert_eq!(1, new.len() - old.len());
}

#[test]
fn test_snake_reset(){
    let snake = Snake::new(1,2).grow().reset();
    let snake2 = Snake::new(1,2);
    assert_eq!(snake,snake2);
}

#[test]
fn test_snake_turn() {
let snake = Snake::new(1,2);
    let snake2 = Snake::new(1,2).turn(Direction::Top);
    assert_eq!(snake.direction, Direction::default());
    assert_eq!(snake2.direction, Direction::Top);
    assert_ne!(snake2.direction, snake.direction);
}

#[test]
fn test_snake_try_eat(){
    let snake = Snake::new(1,2);
    let point = snake.head();
    let point2 = Point{x:100, y:100};

    let len1 = snake.points.len();
    let snake = snake.try_eat(&point2);
    let len2 = snake.points.len();
    let len3 = snake.try_eat(&point).points.len();
    assert_eq!(len1,len2);
    assert_eq!(1, len3 - len2);
}

#[test]
fn test_snake_try_intersect_frame_true(){
    let snake = Snake::new(1,2).grow();
    let snake2 = Snake::new(1,2);
    let head = snake.head();
    let frame = Frame{
        min_x: 1,
        min_y: 1,
        max_x: head.x,
        max_y: head.y
    };
    let snake = snake.try_intersect_frame(&frame);
    assert_eq!(snake,snake2);
}

#[test]
fn test_snake_try_intersect_frame_false(){
    let snake = Snake::new(1,2).grow();
    let snake2 = Snake::new(1,2);
    let frame = Frame{
        min_x: 1,
        min_y: 1,
        max_x: 255,
        max_y: 255
    };
    let snake = snake.try_intersect_frame(&frame);
    assert_ne!(snake,snake2);
    assert_eq!(1, snake.points.len() - snake2.points.len());
}

#[test]
fn test_try_intersect_tail_true(){
    let mut snake = Snake::new(1,2).grow();
    let snake2 = Snake::new(1,2);
    let head = snake.head();
    snake.points.push_back(head);
    let snake = snake.try_intersect_tali();
    assert_eq!(snake, snake2);
}

#[test]
fn test_try_intersect_tail_false(){
    let mut snake = Snake::new(1,2).grow();
    let snake2 = Snake::new(1,2);
    let snake = snake.try_intersect_tali();
    assert_ne!(snake,snake2);
    assert_eq!(1, snake.points.len() - snake2.points.len());
}

#[test]
fn test_mov_snake_should_move_snake_to_1_on_direction(){
    let snake = Snake::new(1,2);
    let movedSnake = snake.clone().move_snake();
    let head = snake.head();
    let movedHead = movedSnake.head();

    assert_eq!(snake.direction, Direction::Right);
    assert_eq!(head.y, movedHead.y);
    assert_eq!(head.x, movedHead.x -1);
}