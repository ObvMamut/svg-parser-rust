# plot_from_file.py
import matplotlib.pyplot as plt
import csv

def plot_points_from_file(filename):
    # Read points from CSV file
    points = []
    with open(filename, 'r') as f:
        reader = csv.reader(f)
        points = [(float(row[0]), float(row[1])) for row in reader]

    # Separate x and y coordinates
    x_coords, y_coords = zip(*points)

    # Create figure and axis
    fig, ax = plt.subplots(figsize=(10, 8))

    # Plot points
    ax.scatter(x_coords, y_coords, color='blue', s=100, alpha=0.6, label='Points')

    # Add grid
    ax.grid(True, linestyle='--', alpha=0.7)

    # Add axis lines at x=0 and y=0
    ax.axhline(y=0, color='k', linestyle='-', alpha=0.3)
    ax.axvline(x=0, color='k', linestyle='-', alpha=0.3)

    # Set labels and title
    ax.set_xlabel('X Coordinate')
    ax.set_ylabel('Y Coordinate')
    ax.set_title('Point Plot')

    # Add legend
    ax.legend()

    # Adjust layout to prevent cutting off labels
    plt.tight_layout()

    plt.gca().set_aspect('equal', adjustable='box')

    # Display the plot
    plt.show()

if __name__ == "__main__":
    plot_points_from_file("points.csv")
