use records::*;
use std::mem;

pub struct PRQuadTree<T: HasPoint> {
    root: Quad<T>,
    max: usize,
    region: Region,
}

type Quad<T> where T: HasPoint = Option<Box<Node<T>>>;

//#[derive(Copy)]
enum Node<T> where T: HasPoint {
    Intr {
        quads: [Quad<T>; 4],
        //quads: Vec<Quad<T>>,
        cur: usize,
        max: usize,
    },
    Leaf {
        elements: Vec<T>,
        cur: usize,
        max: usize,
    },
}

/// Still considering having the Option returns be Result
/// This way debugging could be easier, as well as adding neat functionality.
/// Inserting will pass a region down to be 'chopped up' into quads.
impl<T> Node<T> where T: HasPoint {

    fn new_leaf(vals: Vec<T>, new_cur: usize, new_max: usize) -> Self {
        Node::Leaf { elements: vec![], cur: 0, max: new_max }
    }

    fn new_intr(r: Region, vals: &mut Vec<T>, new_max: usize) -> Self {
        let mut intr =
            Node::Intr { quads: [None, None, None, None], cur: 0, max: new_max };
        while let Some(val) = vals.pop()
            { intr.push(r.clone().quad(val.point()), val); };
        intr
    }

    /// used for decomp / recomp, it's O(n), but with a big coefficient. for now.
    /// Note: Decomposition is currently cloning. This bad.
    fn switch(&mut self, r: Region) {
        let mut new_self = match self
        {
            &mut Node::Intr { quads: _,  cur: c, max: m }
                            => Self::new_leaf(self.vals(), c, m),
            &mut Node::Leaf { elements: ref mut leaf_vals, cur: _, max: m }
                            => Self::new_intr(r, leaf_vals, m)
        };
        mem::replace(self, new_self);
    }

    /// uhhhh.... if it compiles it'll work, right?
    /// Used for into_iter and merge
    fn into_vals(&mut self, mut vals: Vec<T>) -> Vec<T> {
        vec![]
        //match self {
            //&mut Node::Intr { quads: ref mut quads, cur: _, max: _ } => {
                //quads.iter_mut()
                    //.filter(|ref quad| quad.is_some())
                    //.map(|quad| quad.take().unwrap().as_mut());
                    ////.fold(vals,
                         ////|mut vals, mut node| node.into_vals(vals))
            //}
            //&mut Node::Leaf { elements: ref mut leaf_vals, cur: _, max: _ } => {
                //vals.append(leaf_vals);
                //panic!("sdf");
                ////vals
            //}
        //}
    }

    pub fn vals(&mut self) -> Vec<T> {
        self.into_vals(vec![])
    }

    fn push(&mut self, r: Region, val: T) -> bool {
        false
    }

    fn pop(&mut self, r: Region, p: Point) -> Option<T> {
        None
    }

    fn peek(&self, r: Region, p: Point) -> Option<&T> {
        None
    }

    /// likely totally useless now that I think about it
    fn as_intr(&self) -> &Self {
        match &self {
            Intr => &self,
            //Leaf => panic!("Failed to access internal, was leaf.")
        }
    }
}

impl<T> PRQuadTree<T> where T: HasPoint {

    /// Where max_pts is max number of non-dupe pts.
    pub fn new(max_pts: usize) -> Self {
        PRQuadTree { root: None, max: max_pts, region: Region::new(2048) }
    }

    pub fn push(&mut self, val: T) -> Option<T> {
        None
    }

    /// pops off, b/c we want the original inserted data.
    pub fn pop(&mut self, p: Point) -> Option<T> {
        None
    }

    /// Seeks out value
    pub fn peek(&self, p: Point) -> Option<&T> {
        self.root.as_ref()
            .and_then(|root| root.peek(self.reg(), p))
    }

    /// This method is more than just a shorthand way of cloning, might
    /// change properties of the region idk.
    fn reg(&self) -> Region {
        self.region.clone()
    }

}

#[cfg(test)]
mod test {

    use records::HasPoint;
    use records::Point;
    use super::PRQuadTree;

    /// We will use this struct to test how data is saved etc.
    #[derive(PartialEq, Debug)]
    struct Pt {
        description: String,
        coordinate: Point,
    }

    impl Pt {
        pub fn new(message: String, location: Point) -> Self {
            Pt {
                description: message,
                coordinate: location,
            }
        }
    }

    impl HasPoint for Pt {
        fn point(&self) -> Point {
            self.coordinate
        }
    }

    #[test]
    fn init() {
        let mut tree = PRQuadTree::new(5);

        tree.push(Pt::new("Hello".to_string(), (5, 3)));
        assert_eq!(tree.peek((5, 4)), None);
    }

    #[test]
    fn push_pop() {

    }

}
