/**
 * Lists the 
 */

pub struct Record {
    key: String,
    description: String,
    coordinate: Point,
}

pub struct Point {
    id: String,
    x: i64,
    y: i64,
}

pub struct Square {
    origin: Point,
    width: u32,
}

pub struct Rectangle {
    origin: Point,
    width: u32,
    height: u32,
}

#[cfg(test)]
mod test {
    use super::*;
}

