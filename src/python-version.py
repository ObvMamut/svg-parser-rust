import xml.etree.ElementTree as ET
import numpy as np
from svg.path import parse_path
import matplotlib.pyplot as plt

def svg_to_points(svg_path, num_points): # Parse the SVG file
    tree = ET.parse(svg_path)
    root = tree.getroot() # Find the path element
    path_elem = root.find(".//{http://www.w3.org/2000/svg}path")
    if path_elem is None:
        raise ValueError("No path element found in the SVG file.") # Get the 'd' attribute (path data)

    path_data = path_elem.get('d') # Parse the path

    # print(path_data)

    path = parse_path(path_data) # Generate points along the path
    points = []
    for i in range(num_points):
        t = i / (num_points - 1)
        point = path.point(t)
        points.append((point.real, point.imag))

    return np.array(points)

def plot_points(points): # Separate x and y coordinates
    x, y = points.T  # Create the plot
    plt.figure(figsize=(10, 10))
    plt.scatter(x, y, s=5)
    plt.title("SVG Path Points")
    plt.xlabel("X")
    plt.ylabel("Y")
    plt.axis('equal') # Ensure equal scaling
    plt.grid(True)
    plt.show() # Example usage

svg_file_path = "src/drawing.svg"
num_points = 1010 # Increased number of points for smoother representation try:

points_array = svg_to_points(svg_file_path, num_points)

#print(points_array)
# print(f"Generated {len(points_array)} points.")
# plot_points(points_array)
