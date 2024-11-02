import matplotlib.pyplot as plt
from svg.path import parse_path
import numpy as np

def visualize_svg_path(path_data, num_points=1000, title="SVG Path"):
    # Parse the path
    path = parse_path(path_data)
    
    # Generate points along the path
    points = []
    for i in range(num_points):
        t = i / (num_points - 1)
        point = path.point(t)
        points.append((point.real, point.imag))
    
    points = np.array(points)
    
    # Plot the points
    plt.plot(points[:, 0], points[:, 1], linewidth=2, label='Path')
    plt.scatter(points[0, 0], points[0, 1], color='green', s=100, label='Start')
    plt.scatter(points[-1, 0], points[-1, 1], color='red', s=100, label='End')
    
    # Add control points grid
    x_points = points[:, 0]
    y_points = points[:, 1]
    plt.grid(True, linestyle='--', alpha=0.5)
    
    plt.title(title)
    plt.xlabel("X")
    plt.ylabel("Y")
    plt.axis('equal')
    plt.legend()

# The two paths
path1 = "M 20 50 C 20 50 20 50 100 50 C 100 50 100 50 150 20 C 150 20 150 20 200 20 C 200 20 200 20 250 20 C 250 20 250 20 250 100 C 250 100 250 100 250 150 C 250 150 300 200 300 200 350 150 C 350 150 320 120 320 120 300 100 C 300 100 250 50 200 30 150 150 C 150 150 200 180 250 200 300 170"
path2 = "M 20 50 L 100 50 l 50 -30 H 200 h 50 V 100 v 50 Q 300 200 350 150 q -30 -30 -50 -50 C 250 50 200 30 150 150 c 50 30 100 50 150 20"

# Create figure with two subplots side by side
plt.figure(figsize=(20, 8))

# First path
plt.subplot(1, 2, 1)
visualize_svg_path(path1, title="Path 1 (All Cubic Bézier)")

# Second path
plt.subplot(1, 2, 2)
visualize_svg_path(path2, title="Path 2 (Mixed Commands)")

plt.tight_layout()
plt.show()

# Create overlay comparison
plt.figure(figsize=(12, 8))
visualize_svg_path(path1, title="Path Comparison")
points2 = np.array([(path.point(t).real, path.point(t).imag) 
                    for t in np.linspace(0, 1, 1000)])
plt.plot(points2[:, 0], points2[:, 1], '--', linewidth=2, label='Path 2')
plt.legend(['Path 1', 'Start', 'End', 'Path 2'])
plt.show()
