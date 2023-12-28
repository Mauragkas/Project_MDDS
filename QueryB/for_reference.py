from scipy.spatial import ConvexHull
import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D

def quickhull_3d(points):
    """Computes the convex hull of a set of 3D points."""
    hull = ConvexHull(points)
    return points[hull.vertices]

# Example usage
np.random.seed(42)  # For consistent random points
points = np.random.rand(100, 3)  # Generate random 3D points
hull_points = quickhull_3d(points)

# Plotting
fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')
ax.scatter(points[:,0], points[:,1], points[:,2])
ax.plot_trisurf(hull_points[:,0], hull_points[:,1], hull_points[:,2], color='y', alpha=0.5)
plt.show()
