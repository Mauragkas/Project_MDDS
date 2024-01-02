#!/usr/bin/env python
class Node:
    def __init__(self, start, end):
        self.start = start
        self.end = end
        self.max_end = end
        self.left = None
        self.right = None

class IntervalTree:
    def __init__(self, intervals):
        self.root = None
        for i in intervals:
            self.insert(i)

    def insert(self, interval, node=None):
        start, end = interval
        if not node:
            node = self.root
        if not node:
            self.root = Node(start, end)
            return
        if start < node.start:
            if not node.left:
                node.left = Node(start, end)
            else:
                self.insert(interval, node.left)
        else:
            if not node.right:
                node.right = Node(start, end)
            else:
                self.insert(interval, node.right)
        node.max_end = max(node.max_end, end)

    def interval_query(self, start, end, node=None, results=None):
        if results is None:
            results = []
        if not node:
            node = self.root
        if node:
            if start <= node.end and end >= node.start:
                results.append((node.start, node.end))
            if node.left and node.left.max_end >= start:
                self.interval_query(start, end, node.left, results)
            if node.right and node.right.start <= end:
                self.interval_query(start, end, node.right, results)
        return results

    def query(self, point, node=None, results=None):
        if results is None:
            results = []
        if not node:
            node = self.root
        if node:
            if point >= node.start and point <= node.end:
                results.append((node.start, node.end))
            if node.left and point <= node.left.max_end:
                self.query(point, node.left, results)
            if node.right:
                self.query(point, node.right, results)
        return results

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

interval_tree = IntervalTree(intervals)

# Overlap search
for query in queries:
    print("Query:", query, end=" ")
    print("Result:", interval_tree.interval_query(*query))

# Stabbing query
for query in queries:
    print("Query:", query[0], end=" ")
    print("Result:", interval_tree.query(query[0]))
