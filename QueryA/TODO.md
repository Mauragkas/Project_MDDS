# TODO List for Managing

## Overview
Implementing Interval and Segment Trees to manage and query time intervals of scientists' publications in DBLP_Record.

## Tasks

### 1. Data Collection 
(this will be done by the web crawler)
- [ ] Gather publication data for each scientist.
- [ ] Format data into time intervals (e.g., [1996, 1998], [2000, 2000]).

### 2. Implement Interval Tree
- [x] Design an Interval Tree structure.
- [x] Implement insertion of intervals.
- [x] Implement searching for overlapping intervals.

### 3. Implement Segment Tree
- [x] Design a Segment Tree structure.
- [x] Implement insertion of segments.
- [x] Implement point queries (e.g., find scientists active in 2010).
- [x] Implement interval queries (e.g., find scientists active between 2008 and 2012).

### 4. Query Implementation
- [x] Create functions for interval queries (e.g., range overlaps).
- [x] Create functions for stabbing queries (e.g., specific years).

### 5. Testing
- [x] Test Interval Tree with various intervals.
- [x] Test Segment Tree with point and interval queries.
- [x] Validate results against expected outcomes.

### 6. Documentation
- [ ] Document the code.
- [ ] Provide examples and use cases in documentation.

## Notes
- Prioritize accuracy and efficiency in implementation.
- Ensure code is well-commented and follows best practices.
