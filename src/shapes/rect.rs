use std::fmt::Display;

use super::{area::Area, collisions::Collidable, circle::Circle};

pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return self.x <= x && self.x + self.width >= x &&
            self.y <= y && self.y + self.height >= y;
    }
}

impl Collidable<Rect> for Rect {
    fn collide(&self, other: &Rect) -> bool {
        for point in other {
            if self.contains_point(point) {
                return true;
            }
        }
        return false;
    }
}

impl Collidable<Circle> for Rect {
    fn collide(&self, other: &Circle) -> bool {
        return self.contains_point((other.x, other.y));
    }
}

impl Area for Rect {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
            width: 10.0,
            height: 10.0,
        }
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Rectangle({}, {}): {}x{}", self.x, self.y, self.width, self.height);
    }
}

pub struct RectIter {
    points: Vec<(f64, f64)>,
    idx: usize,
}

impl Iterator for RectIter {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.points.len() {
            return None;
        }
        let point = self.points[self.idx];
        self.idx += 1;

        return Some(point);
    }
}

impl From<&Rect> for RectIter {
    fn from(rect: &Rect) -> Self {
        return Self {
            points: vec![
                (rect.x, rect.y),
                (rect.x + rect.width, rect.y),
                (rect.x, rect.y + rect.height),
                (rect.x + rect.width, rect.y + rect.height),
            ],
            idx: 0,
        };
    }
}

impl IntoIterator for Rect {
    type Item = (f64, f64);

    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        return (&self).into();
    }
}

impl IntoIterator for &Rect {
    type Item = (f64, f64);

    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        return self.into();
    }
}