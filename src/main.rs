use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::process::Command;

/// A point structure that can be hashed and compared with floating-point tolerance
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 1e-10;
        (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Round to a specific precision before hashing
        let precision = 1e10;
        let x = (self.x * precision).round() / precision;
        let y = (self.y * precision).round() / precision;

        // Convert to bits for consistent hashing
        x.to_bits().hash(state);
        y.to_bits().hash(state);
    }
}

/// Removes duplicate points from a vector of (f64, f64) coordinates using a HashSet
fn remove_duplicates(points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    let points_set: HashSet<Point> = points.into_iter().map(|(x, y)| Point { x, y }).collect();

    points_set.into_iter().map(|p| (p.x, p.y)).collect()
}

fn save_points_to_file(points: Vec<(f64, f64)>, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    for (x, y) in points {
        writeln!(file, "{},{}", x, y)?;
    }
    Ok(())
}

fn run_python_script() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running Python plotter script...");

    let output = Command::new("python").arg("src/plotter.py").output()?;

    // Print stdout if any
    if !output.stdout.is_empty() {
        println!("Python output: {}", String::from_utf8_lossy(&output.stdout));
    }

    if output.status.success() {
        println!("Python script completed successfully");
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("Python error output: {}", error);
        Err(error.to_string().into())
    }
}

struct Path<'a> {
    path: &'a str,
    commands: Vec<String>,
    points: Vec<(f64, f64)>,
    stack: Vec<f64>,
    synth_commands: Vec<Vec<String>>,
    cartesian_commands: Vec<Vec<String>>,
    n: f64,
    total_length: f64,
}

impl<'a> Path<'a> {
    fn init(path: &'a str) -> Self {
        Path {
            path,
            commands: Self::parse_path_string(path),
            points: vec![],
            stack: vec![],
            synth_commands: vec![],
            cartesian_commands: vec![],
            n: 1000.0,
            total_length: 0.0,
        }
    }

    fn parse_path_string(path: &str) -> Vec<String> {
        // Split the string on spaces and filter out empty strings
        let tokens: Vec<String> = path
            .split_whitespace()
            // Handle negative numbers that might be stuck to commands
            .flat_map(|token| {
                if token.len() > 1 && token.starts_with(|c: char| c.is_alphabetic()) {
                    let (command, number) = token.split_at(1);
                    if number.starts_with('-') {
                        vec![command.to_string(), number.to_string()]
                    } else {
                        vec![token.to_string()]
                    }
                } else {
                    vec![token.to_string()]
                }
            })
            .collect();

        tokens
    }

    fn synthesize(&mut self) {
        // iterate through every term and transform every command into Cubic/Quadratic Bezier Curve

        let mut pointer = 0;

        for command in self.commands.clone() {
            match command.as_str() {
                "M" => {
                    // create the synth command

                    let synth_cmd: Vec<String> = vec![
                        self.commands.clone()[pointer].clone(),
                        self.commands.clone()[pointer + 1].clone(),
                        self.commands.clone()[pointer + 2].clone(),
                    ];
                    self.synth_commands.push(synth_cmd);

                    // update stack

                    self.stack = vec![
                        self.commands.clone()[pointer + 1]
                            .parse()
                            .expect("not valid nr"),
                        self.commands.clone()[pointer + 2]
                            .parse()
                            .expect("not valid nr"),
                    ];
                    pointer += 1;
                }
                "m" => {
                    // create the synth command

                    let synth_cmd: Vec<String> = vec![
                        "M".to_string(),
                        (self.commands.clone()[pointer + 1]
                            .parse::<f64>()
                            .expect("not valid nr")
                            + self.stack[0])
                            .to_string(),
                        (self.commands.clone()[pointer + 2]
                            .parse::<f64>()
                            .expect("not valid nr")
                            + self.stack[1])
                            .to_string(),
                    ];
                    self.synth_commands.push(synth_cmd.clone());

                    // update stack

                    self.stack = vec![
                        synth_cmd[1].parse().expect("not valid nr"),
                        synth_cmd[2].parse().expect("not valid nr"),
                    ];
                    pointer += 1;
                }
                "L" => {
                    let cb_curve = self.transform_line_to_cbezier(
                        self.stack.clone(),
                        vec![
                            self.commands.clone()[pointer + 1]
                                .parse::<f64>()
                                .expect("not valid nr")
                                .to_string(),
                            self.commands.clone()[pointer + 2]
                                .parse::<f64>()
                                .expect("not valid nr")
                                .to_string(),
                        ],
                    );

                    self.synth_commands.push(cb_curve.clone());

                    // update stack

                    self.stack = vec![
                        cb_curve[cb_curve.len() - 2].parse().expect("not valid nr"),
                        cb_curve[cb_curve.len() - 1].parse().expect("not valid nr"),
                    ];
                    pointer += 1;
                }
                "l" => {
                    let cb_curve = self.transform_line_to_cbezier(
                        self.stack.clone(),
                        vec![
                            (self.commands.clone()[pointer + 1]
                                .parse::<f64>()
                                .expect("not valid nr")
                                + self.stack[0])
                                .to_string(),
                            (self.commands.clone()[pointer + 2]
                                .parse::<f64>()
                                .expect("not valid nr")
                                + self.stack[1])
                                .to_string(),
                        ],
                    );

                    self.synth_commands.push(cb_curve.clone());

                    // update stack

                    self.stack = vec![
                        cb_curve[cb_curve.len() - 2].parse().expect("not valid nr"),
                        cb_curve[cb_curve.len() - 1].parse().expect("not valid nr"),
                    ];
                    pointer += 1;
                }
                "H" => {
                    // horizontal line

                    let cb_curve = self.transform_line_to_cbezier(
                        self.stack.clone(),
                        vec![
                            (self.commands.clone()[pointer + 1]
                                .parse::<f64>()
                                .expect("not valid nr"))
                            .to_string(),
                            (self.stack[1]).to_string(), // stack[1] = y, when horizontal line y = const
                        ],
                    );

                    self.synth_commands.push(cb_curve.clone());

                    // update stack

                    self.stack = vec![
                        cb_curve[cb_curve.len() - 2].parse().expect("not valid nr"),
                        cb_curve[cb_curve.len() - 1].parse().expect("not valid nr"),
                    ];
                    pointer += 1;
                }
                "h" => {
                    // horizontal line relative

                    let cb_curve = self.transform_line_to_cbezier(
                        self.stack.clone(),
                        vec![
                            (self.commands.clone()[pointer + 1]
                                .parse::<f64>()
                                .expect("not valid nr")
                                + self.stack[0])
                                .to_string(),
                            (self.stack[1]).to_string(), // stack[1] = y, when horizontal line y = const
                        ],
                    );

                    self.synth_commands.push(cb_curve.clone());

                    // update stack

                    self.stack = vec![
                        cb_curve[cb_curve.len() - 2].parse().expect("not valid nr"),
                        cb_curve[cb_curve.len() - 1].parse().expect("not valid nr"),
                    ];
                    pointer += 1;
                }
                "V" => {
                    // vertical line

                    let cb_curve = self.transform_line_to_cbezier(
                        self.stack.clone(),
                        vec![
                            (self.stack[0]).to_string(), // stack[0] = x, when vertical line x = const
                            (self.commands.clone()[pointer + 1]
                                .parse::<f64>()
                                .expect("not valid nr"))
                            .to_string(),
                        ],
                    );

                    self.synth_commands.push(cb_curve.clone());

                    // upadate stack

                    self.stack = vec![
                        cb_curve[cb_curve.len() - 2].parse().expect("not valid nr"),
                        cb_curve[cb_curve.len() - 1].parse().expect("not valid nr"),
                    ];
                    pointer += 1;
                }
                "v" => {
                    // vertical line relative

                    let cb_curve = self.transform_line_to_cbezier(
                        self.stack.clone(),
                        vec![
                            (self.stack[0]).to_string(), // stack[0] = x, when vertical line x = const
                            (self.commands.clone()[pointer + 1]
                                .parse::<f64>()
                                .expect("not valid nr")
                                + self.stack[1])
                                .to_string(),
                        ],
                    );

                    self.synth_commands.push(cb_curve.clone());

                    // update stack

                    self.stack = vec![
                        cb_curve[cb_curve.len() - 2].parse().expect("not valid nr"),
                        cb_curve[cb_curve.len() - 1].parse().expect("not valid nr"),
                    ];
                    pointer += 1;
                }
                "Q" => {
                    // Quadratic bezier

                    let cb_curve = vec![
                        "Q".to_string(),
                        (self.stack[0].to_string()),
                        (self.stack[1].to_string()),
                        (self.commands.clone()[pointer + 1]
                            .parse::<f64>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 2]
                            .parse::<f64>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 3]
                            .parse::<f64>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 4]
                            .parse::<f64>()
                            .expect("not valid nr"))
                        .to_string(),
                    ];

                    self.synth_commands.push(cb_curve.clone());

                    // upadate stack

                    self.stack = vec![
                        cb_curve[cb_curve.len() - 2].parse().expect("not valid nr"),
                        cb_curve[cb_curve.len() - 1].parse().expect("not valid nr"),
                    ];
                    pointer += 1;
                }
                "q" => {
                    // Quadratic bezier relative

                    let cb_curve = vec![
                        "Q".to_string(),
                        (self.stack[0].to_string()),
                        (self.stack[1].to_string()),
                        (self.commands.clone()[pointer + 1]
                            .parse::<f64>()
                            .expect("not valid nr")
                            + self.stack[0])
                            .to_string(),
                        (self.commands.clone()[pointer + 2]
                            .parse::<f64>()
                            .expect("not valid nr")
                            + self.stack[1])
                            .to_string(),
                        (self.commands.clone()[pointer + 3]
                            .parse::<f64>()
                            .expect("not valid nr")
                            + self.stack[0])
                            .to_string(),
                        (self.commands.clone()[pointer + 4]
                            .parse::<f64>()
                            .expect("not valid nr")
                            + self.stack[1])
                            .to_string(),
                    ];

                    self.synth_commands.push(cb_curve.clone());

                    // upadate stack

                    self.stack = vec![
                        cb_curve[cb_curve.len() - 2].parse().expect("not valid nr"),
                        cb_curve[cb_curve.len() - 1].parse().expect("not valid nr"),
                    ];
                    pointer += 1;
                }
                "C" => {
                    // cubic bezier

                    let cb_curve = vec![
                        "C".to_string(),
                        self.stack[0].to_string(),
                        self.stack[1].to_string(),
                        (self.commands.clone()[pointer + 1]
                            .parse::<f64>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 2]
                            .parse::<f64>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 3]
                            .parse::<f64>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 4]
                            .parse::<f64>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 5]
                            .parse::<f64>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 6]
                            .parse::<f64>()
                            .expect("not valid nr"))
                        .to_string(),
                    ];

                    self.synth_commands.push(cb_curve.clone());

                    // upadate stack

                    self.stack = vec![
                        cb_curve[cb_curve.len() - 2].parse().expect("not valid nr"),
                        cb_curve[cb_curve.len() - 1].parse().expect("not valid nr"),
                    ];
                    pointer += 1;
                }
                "c" => {
                    // cubic bezier relative

                    let cb_curve = vec![
                        "C".to_string(),
                        self.stack[0].to_string(),
                        self.stack[1].to_string(),
                        (self.commands.clone()[pointer + 1]
                            .parse::<f64>()
                            .expect("not valid nr")
                            + self.stack[0])
                            .to_string(),
                        (self.commands.clone()[pointer + 2]
                            .parse::<f64>()
                            .expect("not valid nr")
                            + self.stack[1])
                            .to_string(),
                        (self.commands.clone()[pointer + 3]
                            .parse::<f64>()
                            .expect("not valid nr")
                            + self.stack[0])
                            .to_string(),
                        (self.commands.clone()[pointer + 4]
                            .parse::<f64>()
                            .expect("not valid nr")
                            + self.stack[1])
                            .to_string(),
                        (self.commands.clone()[pointer + 5]
                            .parse::<f64>()
                            .expect("not valid nr")
                            + self.stack[0])
                            .to_string(),
                        (self.commands.clone()[pointer + 6]
                            .parse::<f64>()
                            .expect("not valid nr")
                            + self.stack[1])
                            .to_string(),
                    ];

                    self.synth_commands.push(cb_curve.clone());

                    // upadate stack

                    self.stack = vec![
                        cb_curve[cb_curve.len() - 2].parse().expect("not valid nr"),
                        cb_curve[cb_curve.len() - 1].parse().expect("not valid nr"),
                    ];
                    pointer += 1;
                }
                _ => {
                    pointer += 1;
                }
            }
        }
    }

    fn get_points(&mut self) {
        // synthesize transform every command except M to a cubic bezier curve all except quadratic
        // bezier curves (because C and Q bezier curves use their control points differently)

        self.synthesize();

        // transform every command into cartesian coordinate

        self.cartesian_commands = self.transform_svg_coordinates_to_cartesian();

        // calculate total length

        self.total_length = self.calcluate_total_length();

        // get middle points

        self.points = self.calculate_all_points();

        // get rid of duplicates

        let new_points_no_duplicates = remove_duplicates(self.points.clone());

        // update points

        self.points = new_points_no_duplicates;
    }

    fn transform_line_to_cbezier(&mut self, stack: Vec<f64>, end: Vec<String>) -> Vec<String> {
        // transform line into cubic bezier C[start, 1control point, 2control point, end]

        let mut cb_curve = vec!["C".to_string()];

        // push stack as start and c1 = start

        for _x in 0..2 {
            cb_curve.push(stack[0].to_string());
            cb_curve.push(stack[1].to_string());
        }

        // push c2, end = end

        for _x in 0..2 {
            cb_curve.push(end[0].clone());
            cb_curve.push(end[1].clone());
        }

        return cb_curve;
    }

    fn transform_svg_coordinates_to_cartesian(&mut self) -> Vec<Vec<String>> {
        let mut cartesian_coordinates = vec![];

        // iterate through every command and transform y values to negative

        for command in self.synth_commands.clone() {
            let mut new_command = vec![];

            for x in 0..command.len() {
                match x {
                    0 => {
                        new_command.push(command[x].clone());
                    }
                    _ => {
                        // case if x is divisible by 2 (so the value corresponding is y)
                        if x % 2 == 0 {
                            let mut y_value = command[x].parse::<f64>().expect("not a valid int");

                            // transform to negative value

                            y_value *= -1.0;

                            new_command.push(y_value.to_string());
                        } else {
                            new_command.push(command[x].clone());
                        }
                    }
                }
            }

            cartesian_coordinates.push(new_command);
        }

        return cartesian_coordinates;
    }

    fn get_cubic_bezier_points(
        &mut self,
        p0: (f64, f64),
        p1: (f64, f64),
        p2: (f64, f64),
        p3: (f64, f64),
        n: f64,
    ) -> Vec<(f64, f64)> {
        let mut points = vec![];

        for t in (0..=n as usize).map(|i| i as f64 / n) {
            let x = (1.0 - t).powi(3) * p0.0
                + 3.0 * (1.0 - t).powi(2) * t * p1.0
                + 3.0 * (1.0 - t) * t.powi(2) * p2.0
                + t.powi(3) * p3.0;
            let y = (1.0 - t).powi(3) * p0.1
                + 3.0 * (1.0 - t).powi(2) * t * p1.1
                + 3.0 * (1.0 - t) * t.powi(2) * p2.1
                + t.powi(3) * p3.1;
            points.push((x, y));
        }

        return points;
    }

    fn get_quadratic_bezier_points(
        &mut self,
        p0: (f64, f64),
        p1: (f64, f64),
        p2: (f64, f64),
        n: f64,
    ) -> Vec<(f64, f64)> {
        let mut points = vec![];

        for t in (0..=n as usize).map(|i| i as f64 / n) {
            let x = (1.0 - t).powi(2) * p0.0 + 2.0 * (1.0 - t) * t * p1.0 + t.powi(2) * p2.0;
            let y = (1.0 - t).powi(2) * p0.1 + 2.0 * (1.0 - t) * t * p1.1 + t.powi(2) * p2.1;
            points.push((x, y));
        }

        return points;
    }

    fn calculate_all_points(&mut self) -> Vec<(f64, f64)> {
        // iterate through every cartesian command and calculate every middle point
        let mut points = vec![];

        for command in self.cartesian_commands.clone() {
            let mut middle_n_points = vec![];

            match command[0].as_str() {
                "C" => {
                    // calculate n, (n_length)/(total_length)

                    let n_length = cubic_bezier_arc_length(
                        (
                            command[1].parse::<f64>().expect("not f64"),
                            command[2].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[3].parse::<f64>().expect("not f64"),
                            command[4].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[5].parse::<f64>().expect("not f64"),
                            command[6].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[7].parse::<f64>().expect("not f64"),
                            command[8].parse::<f64>().expect("not f64"),
                        ),
                    );

                    let n_coeff = (n_length / self.total_length.clone());

                    let n = (self.n * n_coeff).round();

                    middle_n_points = self.get_cubic_bezier_points(
                        (
                            command[1].parse::<f64>().expect("not f64"),
                            command[2].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[3].parse::<f64>().expect("not f64"),
                            command[4].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[5].parse::<f64>().expect("not f64"),
                            command[6].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[7].parse::<f64>().expect("not f64"),
                            command[8].parse::<f64>().expect("not f64"),
                        ),
                        n,
                    );
                }
                "Q" => {
                    // calculate n, (n_length)/(total_length)

                    let n_length = quadratic_bezier_arc_length(
                        (
                            command[1].parse::<f64>().expect("not f64"),
                            command[2].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[3].parse::<f64>().expect("not f64"),
                            command[4].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[5].parse::<f64>().expect("not f64"),
                            command[6].parse::<f64>().expect("not f64"),
                        ),
                    );

                    let n_coeff = (n_length / self.total_length.clone());

                    let n = (self.n * n_coeff).round();

                    middle_n_points = self.get_quadratic_bezier_points(
                        (
                            command[1].parse::<f64>().expect("not f64"),
                            command[2].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[3].parse::<f64>().expect("not f64"),
                            command[4].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[5].parse::<f64>().expect("not f64"),
                            command[6].parse::<f64>().expect("not f64"),
                        ),
                        n,
                    );
                }
                _ => {}
            }

            // iterate through middle_n_points extract values add to points, return points

            for coordinates in middle_n_points {
                points.push(coordinates);
            }
        }

        return points;
    }

    fn calcluate_total_length(&mut self) -> f64 {
        let mut length = 0.0;

        for command in self.cartesian_commands.clone() {
            match command[0].as_str() {
                "C" => {
                    let cubic_length = cubic_bezier_arc_length(
                        (
                            command[1].parse::<f64>().expect("not f64"),
                            command[2].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[3].parse::<f64>().expect("not f64"),
                            command[4].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[5].parse::<f64>().expect("not f64"),
                            command[6].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[7].parse::<f64>().expect("not f64"),
                            command[8].parse::<f64>().expect("not f64"),
                        ),
                    );

                    length += cubic_length;
                }
                "Q" => {
                    let quadratic_length = quadratic_bezier_arc_length(
                        (
                            command[1].parse::<f64>().expect("not f64"),
                            command[2].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[3].parse::<f64>().expect("not f64"),
                            command[4].parse::<f64>().expect("not f64"),
                        ),
                        (
                            command[5].parse::<f64>().expect("not f64"),
                            command[6].parse::<f64>().expect("not f64"),
                        ),
                    );

                    length += quadratic_length;
                }
                _ => {}
            }
        }

        return length;
    }
}

fn cubic_bezier_arc_length(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) -> f64 {
    // Gaussian quadrature points and weights for n=7
    const GAUSS_POINTS: [(f64, f64); 7] = [
        (-0.949107912342759, 0.129484966168870),
        (-0.741531185599394, 0.279705391489277),
        (-0.405845151377397, 0.381830050505119),
        (0.000000000000000, 0.417959183673469),
        (0.405845151377397, 0.381830050505119),
        (0.741531185599394, 0.279705391489277),
        (0.949107912342759, 0.129484966168870),
    ];

    // Helper function to calculate position at parameter t
    fn bezier_derivative(
        t: f64,
        p0: (f64, f64),
        p1: (f64, f64),
        p2: (f64, f64),
        p3: (f64, f64),
    ) -> (f64, f64) {
        let t2 = t * t;

        // First calculate the coefficients for the derivative
        let cx = 3.0 * (p1.0 - p0.0);
        let bx = 3.0 * (p2.0 - p1.0) - cx;
        let ax = p3.0 - p0.0 - cx - bx;

        let cy = 3.0 * (p1.1 - p0.1);
        let by = 3.0 * (p2.1 - p1.1) - cy;
        let ay = p3.1 - p0.1 - cy - by;

        // Calculate the derivative at parameter t
        let dx = 3.0 * ax * t2 + 2.0 * bx * t + cx;
        let dy = 3.0 * ay * t2 + 2.0 * by * t + cy;

        (dx, dy)
    }

    // Integrate using Gaussian quadrature
    let arc_length: f64 = GAUSS_POINTS
        .iter()
        .map(|(x, w)| {
            // Transform integration bounds from [-1, 1] to [0, 1]
            let t = (x + 1.0) / 2.0;

            // Calculate derivative at point t
            let (dx, dy) = bezier_derivative(t, p0, p1, p2, p3);

            // Calculate speed at point t
            let speed = (dx * dx + dy * dy).sqrt();

            // Adjust weight for transformed bounds
            speed * w * 0.5
        })
        .sum();

    arc_length
}

fn quadratic_bezier_arc_length(p0: (f64, f64), p1: (f64, f64), p2: (f64, f64)) -> f64 {
    // Gaussian quadrature points and weights for n=7
    const GAUSS_POINTS: [(f64, f64); 7] = [
        (-0.949107912342759, 0.129484966168870),
        (-0.741531185599394, 0.279705391489277),
        (-0.405845151377397, 0.381830050505119),
        (0.000000000000000, 0.417959183673469),
        (0.405845151377397, 0.381830050505119),
        (0.741531185599394, 0.279705391489277),
        (0.949107912342759, 0.129484966168870),
    ];

    // Helper function to calculate derivative at parameter t
    fn bezier_derivative(t: f64, p0: (f64, f64), p1: (f64, f64), p2: (f64, f64)) -> (f64, f64) {
        // For quadratic Bezier:
        // B'(t) = 2(1-t)(P1-P0) + 2t(P2-P1)
        let mt = 1.0 - t;

        let dx = 2.0 * (mt * (p1.0 - p0.0) + t * (p2.0 - p1.0));
        let dy = 2.0 * (mt * (p1.1 - p0.1) + t * (p2.1 - p1.1));

        (dx, dy)
    }

    // Integrate using Gaussian quadrature
    let arc_length: f64 = GAUSS_POINTS
        .iter()
        .map(|(x, w)| {
            // Transform integration bounds from [-1, 1] to [0, 1]
            let t = (x + 1.0) / 2.0;

            // Calculate derivative at point t
            let (dx, dy) = bezier_derivative(t, p0, p1, p2);

            // Calculate speed at point t
            let speed = (dx * dx + dy * dy).sqrt();

            // Adjust weight for transformed bounds
            speed * w * 0.5
        })
        .sum();

    arc_length
}

fn main() {
    let path = "M 20.5 50.0 L 100.0 50.0 l 50.0 -30.0 H 200.0 h 50.0 V 100.0 v 50.0 Q 300.0 200.0 350.0 150.0 q -30.0 -30.0 -50.0 -50.0 C 250.0 50.0 200.0 30.0 150.0 150.0 c 50.0 30.0 100.0 50.0 150.0 20.0";
    let mut pth = Path::init(path);

    pth.get_points();

    let points = pth.points;

    println!("{:?}", points.len());

    let mut length = cubic_bezier_arc_length((0.0, 0.0), (0.0, 0.0), (1.0, 1.0), (1.0, 1.0));

    println!("{:?}", length);

    length = quadratic_bezier_arc_length((0.0, 0.0), (0.5, 0.5), (1.0, 1.0));

    println!("{:?}", length);

    let _error = save_points_to_file(points, "points.csv");

    let _error = run_python_script();
}
