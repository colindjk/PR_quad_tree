
pub struct PRQuadTree<T: Point> {
    root: Quad<T>,
    max: usize,
}

pub trait Point {
    fn point(&self) -> (i64, i64);
}

/// The Quad type, which holds values that implement
/// the 'point' method, might get renamed.
type Quad<T> where T: Point = Option<Box<Node<T>>>;

enum Node<T> {
    Intr {
        ne: Quad<T>,
        nw: Quad<T>,
        se: Quad<T>,
        sw: Quad<T>,
    },
    Leaf {
        elements: Box<Vec<T>>,
    },
}

impl<T: Point> Node<T> {
    pub fn peek(&self, point: (i64, i64)) -> Option<&T> {
        None
    }
}

impl<T: Point> PRQuadTree<T> {

    /// Where max_pts is max number of non-dupe pts.
    pub fn new(max_pts: usize) -> Self {
        PRQuadTree { root: None, max: max_pts, }
    }

    /// Insertion
    pub fn push(&self, val: T) -> bool {
        false
    }

    /// pops off, b/c we want the original inserted data.
    pub fn pop(&self, point: (i64, i64)) -> Option<T> {
        None
    }

    /// Seeks out value
    pub fn peek(&self, point: (i64, i64)) -> Option<&T> {
        self.root.as_ref().and_then(|root| root.peek(point))
    }

}

#[cfg(test)]
mod test {

    use super::Point;
    use super::PRQuadTree as Tree;

    /// We will use this struct to test how data is saved etc.
    #[derive(Debug)]
    #[derive(PartialEq)]
    struct Pt {
        description: String,
        coordinate: (i64, i64),
    }

    impl Pt {
        pub fn new(message: String, location: (i64, i64)) -> Self {
            Pt {
                description: message,
                coordinate: location,
            }
        }
    }

    impl Point for Pt {
        fn point(&self) -> (i64, i64) {
            self.coordinate
        }
    }

    #[test]
    fn init_test() {
        let mut tree = Tree::new(5);

        tree.push(Pt::new("Hello".to_string(), (5, 3)));
        assert_eq!(tree.peek((5, 4)), None);
    }

}
