
/// Sort of the global var which determines what
/// type of integers you want to use.
pub type NumType = u64;

/**
 * Lists the things
 */

pub trait HasPoint {
    fn point(&self) -> (NumType, NumType);
}

pub type Point = (NumType, NumType);

#[derive(Clone, Copy)]
pub struct Region {
    size: NumType,
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

    pub fn new(new_size: NumType) -> Self {
        Region { size: new_size, origin: (0, 0) }
    }

    #[allow(unused)]
    /// modifies the Region into one quarter the size.
    /// Warning, this will make no bounds check for now.
    pub fn quad(&self, p: Point) -> Self {
        Self::new(self.size)
    }
}

#[allow(unused)]
pub struct Record<T: HasPoint> {
    key: String,
    description: String,
    coordinate: T,
}

#[allow(unused)]
pub struct Coordinate {
    id: String,
    x: i64,
    y: i64,
}

#[allow(unused)]
pub struct Square {
    size: NumType,
}

#[allow(unused)]
pub struct Rectangle {
    origin: u32,
    width: u32,
    height: u32,
}

#[cfg(test)]
#[allow(unused)]
mod test {
    use super::*;
}

