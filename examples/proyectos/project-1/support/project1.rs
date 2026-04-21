#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Polygon {
    pub exterior: Vec<Point>,
}

impl Polygon {
    pub fn new(points: Vec<Point>) -> Self {
        Polygon { exterior: points }
    }

    pub(crate) fn area_shoelace(&self) -> f64 {
        let points = &self.exterior;
        let n = points.len();

        if n < 3 {
            return 0.0;
        }

        let mut sum = 0.0;

        for i in 0..n {
            let j = (i + 1) % n;

            sum += points[i].x * points[j].y;
            sum -= points[j].x * points[i].y;
        }

        (sum / 2.0).abs()
    }

    pub fn area(&self) -> f64 {
        self.area_shoelace()
    }

    pub fn new_checked(points: Vec<Point>) -> Option<Self> {
        if points.len() < 4 {
            return None;
        }

        let first = points.first().unwrap();
        let last = points.last().unwrap();

        if (first.x - last.x).abs() > 1e-6 || (first.y - last.y).abs() > 1e-6 {
            return None;
        }

        Some(Polygon { exterior: points })
    }

    pub fn to_wkt(&self) -> String {
        let coords: Vec<String> = self
            .exterior
            .iter()
            .map(|p| format!("{} {}", p.x, p.y))
            .collect();

        format!("POLYGON(({}))", coords.join(", "))
    }
}

pub trait SpatialObject {
    fn area(&self) -> f64;

    fn geometry_type(&self) -> &str;
}

impl SpatialObject for Polygon {
    fn area(&self) -> f64 {
        self.area_shoelace()
    }

    fn geometry_type(&self) -> &str {
        "Polygon"
    }
}

#[allow(dead_code)]
fn main() {}
