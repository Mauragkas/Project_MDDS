#!/usr/bin/env python

class SegmentTreeNode:
    def __init__(self, start, end):
        self.start = start
        self.end = end
        self.intervals = []
        self.left = self.right = None

class SegmentTree:
    def __init__(self, intervals):
        def create_tree(start, end):
            if start > end:
                return None
            node = SegmentTreeNode(start, end)
            if start == end:
                node.intervals = [i for i in intervals if i[0] <= start <= i[1]]
            else:
                mid = (start + end) // 2
                node.left = create_tree(start, mid)
                node.right = create_tree(mid + 1, end)
                node.intervals = sorted(
                    node.left.intervals + node.right.intervals,
                    key=lambda x: x[0]
                )
            return node

        self.root = create_tree(min(i[0] for i in intervals), max(i[1] for i in intervals))

    def query(self, node, point):
        if not node or point < node.start or point > node.end:
            return []
        if node.start == node.end:
            return node.intervals
        return self.unique_intervals(
            self.query(node.left, point) + self.query(node.right, point))

    def interval_query(self, node, start, end):
        if not node or start > node.end or end < node.start:
            return []
        if start <= node.start and end >= node.end:
            return node.intervals
        return self.unique_intervals(
            self.interval_query(node.left, start, end) + self.interval_query(node.right, start, end))

    @staticmethod
    def unique_intervals(intervals):
        return list(set(intervals))

# Example usage
intervals = [
    (1996, 1998), (1997, 1999), # Overlapping
    (2000, 2000), 
    (2003, 2005), (2004, 2006), # Overlapping
    (2008, 2010), (2009, 2011), # Overlapping
    (2015, 2017), (2016, 2018), # Overlapping
    (2022, 2022),
]

queries = [
    (1995, 1997),
    (1998, 2000),
    (2004, 2005),
    (2009, 2012),
    (2016, 2017),
    (2021, 2023),
]


st = SegmentTree(intervals)

for query in queries:
    print('Query:', query, end=' ')
    print('Result:', st.interval_query(st.root, query[0], query[1]))

for query in queries:
    print('Query:', query[0], end=' ')
    print('Result:', st.query(st.root, query[0]))
