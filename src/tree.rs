
pub struct Tree {
    root: Box<Quad>,

}

//const struct 

/// the enum for the quad nodes / leaves.
pub enum Node {
    intr{
        NE: Box<Quad>,
        NW: Box<Quad>,
        SE: Box<Quad>,
        SW: Box<Quad>,
    },
    leaf{

    },
    empt,
}

trait Quad {

}
