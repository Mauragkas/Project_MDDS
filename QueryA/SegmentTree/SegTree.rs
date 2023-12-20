struct SegmentTreeNode {
    start: i32,
    end: i32,
    overlap_count: i32,
    left: Option<Box<SegmentTreeNode>>,
    right: Option<Box<SegmentTreeNode>>,
}

impl SegmentTreeNode {
    fn new(start: i32, end: i32) -> Self {
        SegmentTreeNode {
            start,
            end,
            overlap_count: 0,
            left: None,
            right: None,
        }
    }

    // Function to insert an interval and update the tree
    fn insert(&mut self, start: i32, end: i32) {
        if self.start >= start && self.end <= end {
            self.overlap_count += 1;
            return;
        }
        let mid = self.start + (self.end - self.start) / 2;
        if start < mid {
            if self.left.is_none() {
                self.left = Some(Box::new(SegmentTreeNode::new(self.start, mid)));
            }
            self.left.as_mut().unwrap().insert(start, end);
        }
        if end > mid {
            if self.right.is_none() {
                self.right = Some(Box::new(SegmentTreeNode::new(mid, self.end)));
            }
            self.right.as_mut().unwrap().insert(start, end);
        }
    }

    // Function to query overlap
    fn query_overlap(&self, start: i32, end: i32) -> i32 {
        if self.start >= end || self.end <= start {
            return 0;
        }
        if self.start >= start && self.end <= end {
            return self.overlap_count;
        }
        let mut overlap = self.overlap_count; // Count the current node's overlaps
        let mid = self.start + (self.end - self.start) / 2;
        if start < mid && self.left.is_some() {
            overlap += self.left.as_ref().unwrap().query_overlap(start, end);
        }
        if end > mid && self.right.is_some() {
            overlap += self.right.as_ref().unwrap().query_overlap(start, end);
        }
        overlap
    }    
}

struct SegmentTree {
    root: Option<Box<SegmentTreeNode>>,
}

impl SegmentTree {
    fn new(start: i32, end: i32) -> Self {
        SegmentTree {
            root: Some(Box::new(SegmentTreeNode::new(start, end))),
        }
    }

    // Public method to insert interval
    pub fn insert(&mut self, start: i32, end: i32) {
        if let Some(root) = self.root.as_mut() {
            root.insert(start, end);
        }
    }

    // Public method to query overlap
    pub fn query_overlap(&self, start: i32, end: i32) -> i32 {
        if let Some(root) = &self.root {
            root.query_overlap(start, end)
        } else {
            0
        }
    }
}

fn main() {
    let mut segment_tree = SegmentTree::new(1990, 2030); // Define a range for the segment tree

    // Insert intervals
    let intervals = vec![
        (1996, 1998), (1997, 1999), // Overlapping
        (2000, 2000), 
        (2003, 2005), (2004, 2006), // Overlapping
        (2008, 2010), (2009, 2011), // Overlapping
        (2015, 2017), (2016, 2018), // Overlapping
        (2022, 2022),
    ];

    for interval in intervals {
        segment_tree.insert(interval.0, interval.1);
    }

    // Perform overlap queries
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
        let overlap_count = segment_tree.query_overlap(query.0, query.1);
        println!("Overlap count for interval {:?}: {}", query, overlap_count);
    }
}
