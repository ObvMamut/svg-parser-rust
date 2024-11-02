# SVG Path Parser and Point Generator

A Rust library for parsing SVG path commands and generating discrete points along the path. This tool transforms SVG path commands into a series of points that can be used for visualization or further processing.

## Features

- Parses standard SVG path commands (M, L, H, V, Q, C and their lowercase variants)
- Transforms all commands into cubic or quadratic Bézier curves
- Converts SVG coordinates to Cartesian coordinate system
- Calculates path lengths using Gaussian quadrature
- Generates evenly distributed points along the path
- Removes duplicate points for cleaner output
- Exports points to CSV format
- Includes Python integration for visualization

## Installation

1. Ensure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/)
2. Clone this repository:
```bash
git clone [repository-url]
cd [repository-name]
```

## Dependencies

The project uses the following Rust standard library components:
- `std::collections::HashSet`
- `std::fs::File`
- `std::hash::{Hash, Hasher}`
- `std::io::Write`
- `std::process::Command`

## Usage

### Basic Example

```rust
fn main() {
    // Create an SVG path string
    let path = "M 20.5 50.0 L 100.0 50.0 Q 300.0 200.0 350.0 150.0";

    // Initialize the path processor
    let mut path_processor = Path::init(path);

    // Generate points along the path
    path_processor.get_points();

    // Get the resulting points
    let points = path_processor.points;

    // Save points to CSV
    let _error = save_points_to_file(points, "points.csv");

    // Run visualization script
    let _error = run_python_script();
}
```

### Path Commands Support

The library supports the following SVG path commands:
- `M/m`: Move to
- `L/l`: Line to
- `H/h`: Horizontal line
- `V/v`: Vertical line
- `Q/q`: Quadratic Bézier curve
- `C/c`: Cubic Bézier curve

### Point Generation

Points are generated along the path using the following process:
1. Commands are synthesized into cubic or quadratic Bézier curves
2. Coordinates are transformed to the Cartesian system
3. Arc lengths are calculated using Gaussian quadrature
4. Points are distributed proportionally to segment lengths
5. Duplicate points are removed

## Technical Details

### Path Length Calculation

The library uses 7-point Gaussian quadrature for accurate path length calculations:
```rust
const GAUSS_POINTS: [(f64, f64); 7] = [
    (-0.949107912342759, 0.129484966168870),
    (-0.741531185599394, 0.279705391489277),
    (-0.405845151377397, 0.381830050505119),
    (0.000000000000000, 0.417959183673469),
    (0.405845151377397, 0.381830050505119),
    (0.741531185599394, 0.279705391489277),
    (0.949107912342759, 0.129484966168870),
];
```

### Point Structure

Points are implemented with floating-point tolerance for comparison and hashing:
```rust
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}
```

## Output

The program generates two types of output:
1. A CSV file containing the generated points
2. A visualization (requires the accompanying Python script)

## Contributing

Contributions are welcome! Some areas for potential improvement:
- Additional SVG path commands support
- More sophisticated point distribution algorithms
- Better error handling
- Additional output formats
- Enhanced visualization options
