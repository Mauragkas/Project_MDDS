struct Node {
    start: i32,
    end: i32,
    max_end: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(start: i32, end: i32) -> Self {
        Node {
            start,
            end,
            max_end: end,
            left: None,
            right: None,
        }
    }
}

struct IntervalTree {
    root: Option<Box<Node>>,
}

impl IntervalTree {
    fn new() -> Self {
        IntervalTree { root: None }
    }

    fn insert(&mut self, start: i32, end: i32) {
        self.root = Self::_insert(self.root.take(), start, end);
    }

    fn _insert(node: Option<Box<Node>>, start: i32, end: i32) -> Option<Box<Node>> {
        match node {
            Some(mut n) => {
                if start < n.start {
                    n.left = Self::_insert(n.left.take(), start, end);
                } else {
                    n.right = Self::_insert(n.right.take(), start, end);
                }
                n.max_end = std::cmp::max(n.max_end, end);
                Some(n)
            }
            None => Some(Box::new(Node::new(start, end))),
        }
    }

    fn overlap_search(&self, start: i32, end: i32) -> Option<(i32, i32)> {
        Self::_overlap_search(&self.root, start, end)
    }

    fn _overlap_search(node: &Option<Box<Node>>, start: i32, end: i32) -> Option<(i32, i32)> {
        match node {
            Some(n) => {
                if start <= n.end && end >= n.start {
                    return Some((n.start, n.end));
                }
                if let Some(l) = &n.left {
                    if l.max_end >= start {
                        return Self::_overlap_search(&n.left, start, end);
                    }
                }
                Self::_overlap_search(&n.right, start, end)
            }
            None => None,
        }
    }
}

fn main() {
    let mut interval_tree = IntervalTree::new();
    let intervals = vec![(1996, 1998), (2000, 2000), (2003, 2005), (2008, 2010), (2022, 2022)];

    for interval in intervals {
        interval_tree.insert(interval.0, interval.1);
    }

    let query_result = interval_tree.overlap_search(2008, 2012);
    println!("Query result: {:?}", query_result);
}
