# Unistroke Recognition

## How it works

The recognition algorithm works as follows:

1. The input points are centered around the origin by calculating the centroid and subtracting its x and y values from each point.
2. The points are scaled to fit within the bounds `[-1, 1]`.
3. The points are resampled to a fixed number of evenly spaced points along the stroke.
4. Each pair of consecutive points is used to calculate the direction of its segment.
5. The input segment directions are compared with the corresponding directions in each stored shape.
6. The direction similarities are averaged into a confidence score between `0.0` and `1.0`.
7. The shapes are returned in descending order of confidence.

## Usage

### Basic Usage

```rust
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
        ]
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

    println!("Best match: {} ({confidence})", best_match.id()); // --> Best match: circle (0.99590284)
```

## Bevy/egui Demo

This project has a companion Bevy application with an `egui` interface for drawing and recognizing unistroke shapes.

See the [Bevy/egui demo repository](https://github.com/ygharsallah/unistroke-recognition-demo).