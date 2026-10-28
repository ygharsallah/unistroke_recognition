use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Shape {
    id: String,
    points: Vec<Point>,
}

impl Shape {
    pub fn new(id: String, points: Vec<Point>) -> Self {
        Self { id, points }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn points(&self) -> &[Point] {
        &self.points
    }
}
