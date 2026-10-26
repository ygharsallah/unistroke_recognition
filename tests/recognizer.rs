use unistroke_recognition::prelude::{Point, Recognizer, Shape};

fn point(x: f32, y: f32) -> Point {
    Point::new(x, y).unwrap()
}

#[test]
fn basic_usage_test() {
    use unistroke_recognition::prelude::*;

    let circle = Shape::new(
        "circle".to_string(),
        vec![
            Point::new(0.0, 0.9).unwrap(),
            Point::new(0.45, 0.75).unwrap(),
            Point::new(0.75, 0.4).unwrap(),
            Point::new(0.9, 0.0).unwrap(),
            Point::new(0.75, -0.4).unwrap(),
            Point::new(0.45, -0.75).unwrap(),
            Point::new(0.0, -0.9).unwrap(),
            Point::new(-0.45, -0.75).unwrap(),
            Point::new(-0.75, -0.4).unwrap(),
            Point::new(-0.9, 0.0).unwrap(),
            Point::new(-0.75, 0.4).unwrap(),
            Point::new(-0.45, 0.75).unwrap(),
            Point::new(0.0, 0.9).unwrap(),
        ],
    );

    let triangle = Shape::new(
        "triangle".to_string(),
        vec![
            Point::new(-0.8, -0.7).unwrap(),
            Point::new(0.0, 0.8).unwrap(),
            Point::new(0.8, -0.7).unwrap(),
            Point::new(-0.8, -0.7).unwrap(),
        ],
    );

    let mut recognizer = Recognizer::new(None);
    recognizer.add_shape(circle);
    recognizer.add_shape(triangle);

    // A sampled stroke drawn around a circle, with a different position and scale.
    let input_points = vec![
        Point::new(0.2, 0.8).unwrap(),
        Point::new(0.65, 0.65).unwrap(),
        Point::new(0.95, 0.3).unwrap(),
        Point::new(1.0, -0.1).unwrap(),
        Point::new(0.85, -0.5).unwrap(),
        Point::new(0.5, -0.8).unwrap(),
        Point::new(0.1, -0.9).unwrap(),
        Point::new(-0.3, -0.75).unwrap(),
        Point::new(-0.6, -0.45).unwrap(),
        Point::new(-0.7, -0.05).unwrap(),
        Point::new(-0.55, 0.35).unwrap(),
        Point::new(-0.2, 0.7).unwrap(),
        Point::new(0.2, 0.8).unwrap(),
    ];

    // Results are sorted from the highest confidence score to the lowest.
    let results = recognizer.parse(&input_points);
    let (best_match, confidence) = &results[0];

    println!("Best match: {} ({confidence})", best_match.id());
}

#[test]
fn recognizes_a_circle_as_more_similar_than_a_triangle() {
    let circle = Shape::new(
        "circle".to_owned(),
        vec![
            point(0.0, 0.9),
            point(0.45, 0.75),
            point(0.75, 0.4),
            point(0.9, 0.0),
            point(0.75, -0.4),
            point(0.45, -0.75),
            point(0.0, -0.9),
            point(-0.45, -0.75),
            point(-0.75, -0.4),
            point(-0.9, 0.0),
            point(-0.75, 0.4),
            point(-0.45, 0.75),
            point(0.0, 0.9),
        ],
    );
    let triangle = Shape::new(
        "triangle".to_owned(),
        vec![
            point(-0.8, -0.7),
            point(0.0, 0.8),
            point(0.8, -0.7),
            point(-0.8, -0.7),
        ],
    );

    let mut recognizer = Recognizer::new(None);
    recognizer.add_shape(circle);
    recognizer.add_shape(triangle);

    let input = vec![
        point(0.2, 0.8),
        point(0.65, 0.65),
        point(0.95, 0.3),
        point(1.0, -0.1),
        point(0.85, -0.5),
        point(0.5, -0.8),
        point(0.1, -0.9),
        point(-0.3, -0.75),
        point(-0.6, -0.45),
        point(-0.7, -0.05),
        point(-0.55, 0.35),
        point(-0.2, 0.7),
        point(0.2, 0.8),
    ];

    let results = recognizer.parse(&input);

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0.id(), "circle");
    assert!(results[0].1 > results[1].1);
}
