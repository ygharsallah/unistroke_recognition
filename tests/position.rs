use unistroke_recognition::prelude::{Point, PointError};

#[test]
fn creates_position_with_valid_coordinates() {
    let position = Point::new(0.25, -0.75).unwrap();

    assert_eq!(position.x(), 0.25);
    assert_eq!(position.y(), -0.75);
    assert_eq!(position.to_string(), "(0.25, -0.75)");
}

#[test]
fn rejects_coordinates_outside_the_allowed_range() {
    assert_eq!(
        Point::new(1.1, 0.0),
        Err(PointError::XOutOfRange { value: 1.1 })
    );
    assert_eq!(
        Point::new(0.0, -1.1),
        Err(PointError::YOutOfRange { value: -1.1 })
    );
}

#[test]
fn rejects_non_finite_coordinates() {
    assert_eq!(Point::new(f32::NAN, 0.0), Err(PointError::NotFinite));
    assert_eq!(Point::new(0.0, f32::INFINITY), Err(PointError::NotFinite));
}
