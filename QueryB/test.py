#!/usr/bin/env python
import numpy as np
import matplotlib.pyplot as plt
from scipy.spatial import ConvexHull
from mpl_toolkits.mplot3d.art3d import Poly3DCollection

# Generate random points
points = np.random.rand(30, 3)

# Compute the convex hull
hull = ConvexHull(points)

# Plotting
fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')

# Plot the points
ax.scatter(points[:,0], points[:,1], points[:,2])

# Plot the convex hull
for simplex in hull.simplices:
    ax.plot(points[simplex, 0], points[simplex, 1], points[simplex, 2], 'k-')

# Add semi-transparent faces
faces = [hull.points[simplex] for simplex in hull.simplices]
face_collection = Poly3DCollection(faces, alpha=0.25, linewidths=1, edgecolors='k')
ax.add_collection3d(face_collection)

plt.show()

