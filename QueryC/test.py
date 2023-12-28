import matplotlib.pyplot as plt
import numpy as np

def orientation(p, q, r):
    """Calculate orientation of ordered triplet (p, q, r). 
    Returns 0 if collinear, 1 if clockwise, 2 if counterclockwise."""
    val = (q[1] - p[1]) * (r[0] - q[0]) - (q[0] - p[0]) * (r[1] - q[1])
    if val == 0: return 0  # Collinear
    return 1 if val > 0 else 2  # Clock or counterclockwise

def convex_hull(points):
    """Perform Graham Scan to find the convex hull of a set of 2D points."""
    n = len(points)
    if n < 3: return  # Convex hull not possible with less than 3 points

    # Find the bottom-most point (or choose the left most point in case of tie)
    l = np.argmin(points[:,1])
    
    points = np.roll(points, -l, axis=0)  # Place the bottom-most point at first position

    # Sort the remaining points based on their angle with the first point
    sorted_pts = sorted(points[1:], key=lambda p: np.arctan2(p[1] - points[0][1], p[0] - points[0][0]))

    # Place the bottom-most point back in the sorted list
    sorted_pts.insert(0, points[0])

    # Create an empty stack and push first three points
    hull = sorted_pts[:3]

    # Process remaining points
    for p in sorted_pts[3:]:
        while len(hull) > 1 and orientation(hull[-2], hull[-1], p) != 2:
            hull.pop()
        hull.append(p)

    return np.array(hull)

# Example usage
points = np.random.rand(100, 2)  # Generate random points
hull_points = convex_hull(points)

# Plotting
plt.scatter(points[:,0], points[:,1])
for i in range(len(hull_points)):
    plt.plot([hull_points[i][0], hull_points[(i+1) % len(hull_points)][0]], 
             [hull_points[i][1], hull_points[(i+1) % len(hull_points)][1]], 'r')
plt.show()
