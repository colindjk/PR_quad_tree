/**
 * Lists the things
 */

pub trait Point {
    fn point(&self) -> (u64, u64);
}

pub struct Region {
    size: u64,
}

/// Figure out how to make an immutable region have bounded regions
/// per quad thank-you.
impl BoundedRegion {
    pub fn quad(&self, &point: (u64, u64)) -> Option<u8> {
        match &point {
            (0, y) => Some(0),
            (x, 0) => Some(self.size),
            //(self.size, 0) => Some(1),
            (x, 0) => Some(1),
            _ => None,
        }
    }
}

pub struct Record<T: Point> {
    key: String,
    description: String,
    coordinate: T,
}

pub struct Coordinate {
    id: String,
    x: i64,
    y: i64,
}

pub struct Square {
    size: u64,
}

pub struct Rectangle {
    origin: u32,
    width: u32,
    height: u32,
}

#[cfg(test)]
mod test {
    use super::*;
}

