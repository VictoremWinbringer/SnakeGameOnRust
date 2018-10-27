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

