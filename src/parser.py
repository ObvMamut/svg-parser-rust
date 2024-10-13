import svgpathtools
import numpy as np

def svg_to_points(svg_path, num_points):
    # Parse the SVG path
    path = svgpathtools.parse_path(svg_path)

    # Calculate total length of the path
    total_length = path.length()

    # Calculate step size
    step_size = total_length / (num_points - 1)

    # Generate points
    points = []
    for i in range(num_points):
        distance = i * step_size
        point = path.point(path.ilength(distance))
        points.append((point.real, point.imag))

    return np.array(points)

# Example usage
svg_path = "M0 0 L100 0 L100 100 L0 100 Z"
num_points = 100
points = svg_to_points(svg_path, num_points)

print(points)

# Now 'points' is a numpy array of (x, y) coordinates
# that can be plotted on a graph
