#!/usr/bin/env python

import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d.art3d import Poly3DCollection
import numpy as np

def plot_convex_hull(points, hull_faces):
    fig = plt.figure()
    ax = fig.add_subplot(111, projection='3d')

    # Plot the original points
    xs, ys, zs = zip(*points)
    ax.scatter(xs, ys, zs)

    # Draw each face of the hull
    for face in hull_faces:
        # Assuming 'face' is a tuple or list of 3 points
        x, y, z = zip(*face)
        vertices = [list(zip(x, y, z))]
        poly3d = Poly3DCollection(vertices, alpha=0.5, facecolors='cyan')
        ax.add_collection3d(poly3d)

    ax.set_xlabel('X Axis')
    ax.set_ylabel('Y Axis')
    ax.set_zlabel('Z Axis')
    plt.title('3D Convex Hull')
    plt.show()

def find_initial_simplex(points):
    # Implement logic to find 4 non-coplanar points
    # This is a simplified approach
    return points[:4]

def distance_from_plane(point, plane_points):
    p1, p2, p3 = [np.array(p) for p in plane_points]
    # Debug: Print the shapes of the vectors
    print("p1, p2, p3 shapes:", p1.shape, p2.shape, p3.shape)

    normal_vector = np.cross(p2 - p1, p3 - p1)
    # Debug: Print the shape of the normal vector
    print("Normal vector shape:", normal_vector.shape)

    # Ensure that the point is a numpy array
    point = np.array(point)
    return np.dot(normal_vector, point - p1) / np.linalg.norm(normal_vector)

def distance_from_plane(point, plane_points):
    # Convert to numpy arrays and ensure 3D points
    p1, p2, p3 = [np.array(p) for p in plane_points]
    print("p1, p2, p3 shapes:", p1.shape, p2.shape, p3.shape)  # Debug print

    # Calculate normal vector
    normal_vector = np.cross(p2 - p1, p3 - p1)
    print("Normal vector shape:", normal_vector.shape)  # Debug print

    # Ensure point is a numpy array
    point = np.array(point)
    return np.dot(normal_vector, point - p1) / np.linalg.norm(normal_vector)

def divide_points_among_faces(points, simplex):
    faces = {}
    for i, face_points in enumerate(simplex):
        # Ensure each face has exactly 3 points
        if len(face_points) != 3:
            raise ValueError(f"Face {i} does not have exactly 3 points: {face_points}")

        # Debug: Print the current face being processed
        print("Processing face:", i, "with points:", face_points)
        faces[i] = [p for p in points if distance_from_plane(p, face_points) > 0]
    return faces

# Ensure the simplex is correctly formed
points = np.random.rand(30, 3)  # Example points
simplex = [points[:3], points[1:4], points[2:5], points[3:6]]  # Example simplex, make sure it's correctly defined
faces = divide_points_among_faces(points, simplex)

# Sample usage
# simplex = [points[:3], points[1:4], points[2:5], points[3:6]]  # Example simplex
simplex = find_initial_simplex(points)
# faces = divide_points_among_faces(points, simplex)
hull_faces = [simplex]

# Plot the convex hull
# plot_convex_hull(points, faces.values())
plot_convex_hull(points, hull_faces)

