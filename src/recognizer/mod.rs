use crate::prelude::*;

pub struct Recognizer {
    shapes: Vec<Shape>,
    normalized_point_count: usize,
}

impl Recognizer {
    pub fn new(normalized_point_count: Option<usize>) -> Self {
        let normalized_point_count =
            normalized_point_count.unwrap_or(crate::config::NORMALIZED_POINT_COUNT);

        if normalized_point_count < 2 {
            panic!("normalized_point_count must be greater than 1");
        }

        Self {
            shapes: Vec::new(),
            normalized_point_count,
        }
    }

    pub fn get_shapes(&self) -> &[Shape] {
        &self.shapes
    }

    pub fn add_shape(&mut self, shape: Shape) {
        let normalized_points: Vec<Point> =
            normalize_points(shape.points(), self.normalized_point_count);
        let id: String = shape.id().to_string();

        let shape: Shape = Shape::new(id, normalized_points);

        if let Some(existing) = self
            .shapes
            .iter()
            .find(|s| s.id() == shape.id() || s.points() == shape.points())
        {
            panic!(
                "Shape with name or points '{}' already exists. Conflict with '{}'",
                shape.id(),
                existing.id()
            );
        }

        self.shapes.push(shape);
    }

    pub fn parse(&self, input_points: &[Point]) -> Vec<(Shape, f32)> {
        let mut shape_confidence: Vec<(Shape, f32)> = vec![];
        let normalized_input_points: Vec<Point> =
            normalize_points(input_points, self.normalized_point_count);

        for shape in &self.shapes {
            let confidence_score: f32 =
                calculate_confidence_for_shape(&normalized_input_points, shape);

            shape_confidence.push((shape.clone(), confidence_score));
        }

        shape_confidence.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        shape_confidence
    }
}

/// Calculates a confidence score from `0.0` to `1.0`, where `1.0` represents the highest similarity.
fn calculate_confidence_for_shape(points: &[Point], shape: &Shape) -> f32 {
    if points.len() < 2 {
        return 0.0;
    }

    if points.len() != shape.points().len() {
        panic!(
            "Input points and shape points must have the same length. Input: {}, Shape: {}",
            points.len(),
            shape.points().len()
        );
    }

    let mut total_similarity = 0.0;
    let mut compared_segments = 0;

    for (input_segment, shape_segment) in points.windows(2).zip(shape.points().windows(2)) {
        let input_delta = calculate_delta(&input_segment[0], &input_segment[1]);
        let shape_delta = calculate_delta(&shape_segment[0], &shape_segment[1]);
        let input_length = calculate_length(&input_delta);
        let shape_length = calculate_length(&shape_delta);

        if input_length == 0.0 || shape_length == 0.0 {
            continue;
        }

        let dot_product =
            calculate_dot_product(&input_delta, &shape_delta) / (input_length * shape_length);
        let directional_similarity = ((dot_product.clamp(-1.0, 1.0)) + 1.0) / 2.0;

        total_similarity += directional_similarity;
        compared_segments += 1;
    }

    if compared_segments == 0 {
        return 0.0;
    }

    total_similarity / compared_segments as f32
}

fn calculate_delta(first_point: &Point, second_point: &Point) -> Point {
    Point::from_coordinates(
        second_point.x() - first_point.x(),
        second_point.y() - first_point.y(),
    )
}

fn calculate_length(point: &Point) -> f32 {
    (point.x() * point.x() + point.y() * point.y()).sqrt()
}

fn calculate_dot_product(first: &Point, second: &Point) -> f32 {
    first.x() * second.x() + first.y() * second.y()
}

/// Centers points around the origin, uniformly scales them to `[-1, 1]` and resamples points to a fixed number of points.
fn normalize_points(points: &[Point], normalized_point_count: usize) -> Vec<Point> {
    if points.is_empty() {
        return Vec::new();
    }

    let centered_points = center_points_at_origin(points);
    let scaled_points = scale_points_to_bounds(&centered_points);

    resample_points(&scaled_points, normalized_point_count)
}

/// Translates the points so their centroid is at the origin.
fn center_points_at_origin(points: &[Point]) -> Vec<Point> {
    let amount_of_points = points.len() as f32;
    let center_x = points.iter().map(Point::x).sum::<f32>() / amount_of_points;
    let center_y = points.iter().map(Point::y).sum::<f32>() / amount_of_points;

    points
        .iter()
        .map(|point| Point::from_coordinates(point.x() - center_x, point.y() - center_y))
        .collect()
}

/// Scales centered points uniformly so every coordinate fits within `[-1, 1]`.
fn scale_points_to_bounds(points: &[Point]) -> Vec<Point> {
    let scale = points
        .iter()
        .map(|point| point.x().abs().max(point.y().abs()))
        .fold(0.0_f32, f32::max);
    let scale = if scale == 0.0 { 1.0 } else { scale };

    points
        .iter()
        .map(|point| {
            Point::new(point.x() / scale, point.y() / scale)
                .expect("scaled point should fit within [-1, 1]")
        })
        .collect()
}

/// Resamples the points to a fixed number of points.
fn resample_points(points: &[Point], num_points: usize) -> Vec<Point> {
    if points.is_empty() || num_points == 0 {
        return Vec::new();
    }

    if num_points == 1 {
        return vec![points[0]];
    }

    if points.len() == 1 {
        return vec![points[0]; num_points];
    }

    let length = calculate_total_length(points);
    if length == 0.0 {
        return vec![points[0]; num_points];
    }

    let interval = length / (num_points - 1) as f32;

    let mut resampled_points = Vec::new();
    let mut current_length = 0.0;
    let mut current_index = 0;

    for i in 0..num_points {
        let target_length = i as f32 * interval;

        while current_index < points.len() - 1 {
            let p1 = points[current_index];
            let p2 = points[current_index + 1];
            let segment_length = calculate_distance(&p1, &p2);

            if segment_length == 0.0 {
                current_index += 1;
                continue;
            }

            if current_length + segment_length >= target_length {
                let t = ((target_length - current_length) / segment_length).clamp(0.0, 1.0);
                resampled_points.push(
                    Point::new(
                        p1.x() + t * (p2.x() - p1.x()).clamp(-1.0, 1.0),
                        p1.y() + t * (p2.y() - p1.y()).clamp(-1.0, 1.0),
                    )
                    .expect("resampled point should contain finite coordinates"),
                );
                break;
            }

            current_length += segment_length;
            current_index += 1;
        }

        if resampled_points.len() < i + 1 {
            resampled_points.push(*points.last().unwrap());
        }
    }

    resampled_points
}

fn calculate_total_length(points: &[Point]) -> f32 {
    points
        .windows(2)
        .map(|window| calculate_distance(&window[0], &window[1]))
        .sum()
}

fn calculate_distance(first: &Point, second: &Point) -> f32 {
    let delta_x = first.x() - second.x();
    let delta_y = first.y() - second.y();

    (delta_x * delta_x + delta_y * delta_y).sqrt()
}
