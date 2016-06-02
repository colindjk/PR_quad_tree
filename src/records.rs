/**
 * Lists the things
 */

pub trait HasPoint {
    fn point(&self) -> (u64, u64);
}

pub type Point = (u64, u64);

#[derive(Clone, Copy)]
pub struct Region {
    size: u64,
    origin: Point,
}

enum Quadrant {
    NE = 0,
    NW = 1,
    SW = 2,
    SE = 3,
}

/// Figure out how to make an immutable region have bounded regions
/// per quad thank-you.
impl Region {

    pub fn new(new_size: u64) -> Self {
        Region { size: new_size, origin: (0, 0) }
    }

    /// modifies the Region into one quarter the size.
    /// Warning, this will make no bounds check for now.
    pub fn quad(&self, p: Point) -> Self {
        Self::new(self.size)
    }
}

pub struct Record<T: HasPoint> {
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

