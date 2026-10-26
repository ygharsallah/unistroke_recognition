use unistroke_recognition::prelude::{Point, Shape};

#[test]
fn creates_shape_with_id_and_points() {
    let points = vec![Point::new(0.0, 0.0).unwrap(), Point::new(1.0, 1.0).unwrap()];
    let shape = Shape::new("line".to_owned(), points.clone());

    assert_eq!(shape.id(), "line");
    assert_eq!(shape.points(), points.as_slice());
}

#[test]
fn exposes_points_without_copying_them() {
    let shape = Shape::new("point".to_owned(), vec![Point::new(0.5, -0.5).unwrap()]);

    assert_eq!(shape.points().len(), 1);
    assert_eq!(shape.points()[0], Point::new(0.5, -0.5).unwrap());
}
