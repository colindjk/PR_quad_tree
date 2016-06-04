
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
    x: NumType,
    y: NumType,
    w: NumType,
    h: NumType,
}

pub enum Quadrant {
    NE = 0,
    NW = 1,
    SW = 2,
    SE = 3,
}

/// Figure out how to make an immutable region have bounded regions
/// per quad thank-you.
impl Region {

    pub fn new(x: NumType, y: NumType, w: NumType, h: NumType) -> Self {
        Region { x: x, y: y, w: w, h: h }
    }

    pub fn contains(&self, p: Point) -> bool {

    }

    pub fn to_quadrant(self, p: Point) -> Self {
        match self.get_quadrant(p) {
            Quadrant::NE => self.ne(),
            Quadrant::NW => self.nw(),
            Quadrant::SW => self.sw(),
            Quadrant::SE => self.se(),
        }
    }

    /// the selfy-ning.

    fn ne(mut self) -> Self {
        self.w = self.w / 2; self.h = self.h / 2;
        self.x = self.x + self.w;
        self.y = self.y + self.h;
        self
    }

    fn nw(mut self) -> Self {
        self.w = self.w / 2; self.h = self.h / 2;
        self.y = self.y + self.h;
        self
    }

    fn se(mut self) -> Self {
        self.w = self.w / 2; self.h = self.h / 2;
        self.x = self.x + self.w;
        self
    }

    fn sw(mut self) -> Self {
        self.w = self.w / 2; self.h = self.h / 2;
        self
    }

    /// WARNING: This method will not check for bounds b/c yolo.
    pub fn get_quadrant(&self, p: Point) -> Quadrant {
        let (x, y) = p;
        if x > self.x + (self.w / 2) {
            if y > self.y + (self.h / 2) {
                Quadrant::NE
            } else { Quadrant::SE }
        } else {
            if y > self.y + (self.h / 2) {
                Quadrant::NW
            } else { Quadrant::SW }
        }
    }

}

impl Default for Region {
    fn default() -> Self {
        Region { x: 0, y: 0, w: 2048, h: 2048 }
    }
}

/// ---| Testing section |---
#[cfg(test)]
mod test {

}

//#[allow(unused)]
//pub struct Record<T: HasPoint> {
    //key: String,
    //description: String,
    //coordinate: T,
//}

//#[allow(unused)]
//pub struct Coordinate {
    //id: String,
    //x: i64,
    //y: i64,
//}

//#[allow(unused)]
//pub struct Square {
    //size: NumType,
//}

//#[allow(unused)]
//pub struct Rectangle {
    //origin: u32,
    //width: u32,
    //height: u32,
//}

//#[cfg(test)]
//#[allow(unused)]
//mod test {
    //use super::*;
//}

