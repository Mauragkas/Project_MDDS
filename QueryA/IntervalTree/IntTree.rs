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

    #[allow(dead_code)]
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

    fn overlap_search_all(&self, start: i32, end: i32) -> Vec<(i32, i32)> {
        let mut results = Vec::new();
        Self::_overlap_search_all(&self.root, start, end, &mut results);
        results
    }

    fn _overlap_search_all(
        node: &Option<Box<Node>>,
        start: i32,
        end: i32,
        results: &mut Vec<(i32, i32)>
    ) {
        if let Some(n) = node {
            if start <= n.end && end >= n.start {
                results.push((n.start, n.end));
            }
            if n.left.is_some() && n.left.as_ref().unwrap().max_end >= start {
                Self::_overlap_search_all(&n.left, start, end, results);
            }
            if n.right.is_some() && (n.right.as_ref().unwrap().start <= end) {
                Self::_overlap_search_all(&n.right, start, end, results);
            }
        }
    }
}

fn main() {
    let mut interval_tree = IntervalTree::new();
    let intervals = vec![
        (1996, 1998), (1997, 1999), // Overlapping
        (2000, 2000), 
        (2003, 2005), (2004, 2006), // Overlapping
        (2008, 2010), (2009, 2011), // Overlapping
        (2015, 2017), (2016, 2018), // Overlapping
        (2022, 2022),
    ];

    for interval in intervals {
        interval_tree.insert(interval.0, interval.1);
    }

    let queries = [
        (1995, 1997),
        (1998, 2000),
        (2004, 2005),
        (2009, 2012),
        // (2007, 2012),
        (2016, 2017),
        (2021, 2023),
    ];

    for query in queries {
        // let result = interval_tree.overlap_search(query.0, query.1);
        let result = interval_tree.overlap_search_all(query.0, query.1);
        println!("Query {:?} result: {:?}", query, result);
    }
}
