use records::*;

pub struct PRQuadTree<T: Point> {
    root: Quad<T>,
    max: u8,
    region: Region,
}

type Quad<T> where T: Point = Option<Box<Node<T>>>;

enum Node<T> {
    Intr {
        children: [Quad<T>; 4],
        max: u8,
    },
    Leaf {
        elements: Vec<T>,
        max: u8,
    },
}

/// Still considering having the Option returns be Result
/// This way debugging could be easier, as well as adding neat functionality.
impl<T> Node<T> where T: Point {

    fn push(&mut self, r: Region, val: T) -> bool {
        false
    }

    fn pop(&mut self, p: (i64, i64)) -> Option<T> {
        None
    }

    fn peek(&self, p: (i64, i64)) -> Option<&T> {
        None
    }

    fn as_intr(&self) -> &Self {
        match &self {
            intr => &self,
            Leaf => panic!("Failed to access internal, was leaf.")
        }
    }
}

impl<T> PRQuadTree<T> where T: Point {
    /// Where max_pts is max number of non-dupe pts.
    pub fn new(max_pts: u8) -> Self {
        PRQuadTree { root: None, max: max_pts, }
    }
    /// Insertion
    fn push(&mut self, val: T) -> bool {
        false
    }

    /// pops off, b/c we want the original inserted data.
    fn pop(&mut self, p: (i64, i64)) -> Option<T> {
        None
    }

    /// Seeks out value
    fn peek(&self, p: (i64, i64)) -> Option<&T> {
        self.root.as_ref().and_then(|root| root.peek(p))
    }
}

#[cfg(test)]
mod test {

    use records::Point;
    use super::PRQuadTree;

    /// We will use this struct to test how data is saved etc.
    #[derive(PartialEq, Debug)]
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
    fn init() {
        let mut tree = PRQuadTree::new(5);

        tree.push(Pt::new("Hello".to_string(), (5, 3)));
        assert_eq!(tree.peek((5, 4)), None);
    }

    #[test]
    fn push_pop() {

    }

}
