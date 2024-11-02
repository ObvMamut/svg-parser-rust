use plotly::Plot;
use plotly::Scatter;

// Scatter Plots
fn scatter_plot(points: Vec<Vec<i32>>) {
    let mut x_values = vec![];
    let mut y_values = vec![];

    for coordinate in points {
        x_values.push(coordinate[0]);
        y_values.push(coordinate[1]);
    }

    // Create a scatter plot
    let trace = Scatter::new(x_values, y_values).mode(plotly::common::Mode::Markers);

    // Create the plot and add the trace
    let mut plot = Plot::new();
    plot.add_trace(trace);

    // Display the plot
    plot.show();
}

struct Path<'a> {
    path: &'a str,
    commands: Vec<String>,
    points: Vec<Vec<i32>>,
    stack: Vec<i32>,
    synth_commands: Vec<Vec<String>>,
    cartesian_commands: Vec<Vec<String>>,
    n: i32,
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
            n: 10,
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
        // iterate through every term and transform every command into Cubic Bezier Curve

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
                            .parse::<i32>()
                            .expect("not valid nr")
                            + self.stack[0])
                            .to_string(),
                        (self.commands.clone()[pointer + 2]
                            .parse::<i32>()
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
                                .parse::<i32>()
                                .expect("not valid nr")
                                .to_string(),
                            self.commands.clone()[pointer + 2]
                                .parse::<i32>()
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
                                .parse::<i32>()
                                .expect("not valid nr")
                                + self.stack[0])
                                .to_string(),
                            (self.commands.clone()[pointer + 2]
                                .parse::<i32>()
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
                                .parse::<i32>()
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
                                .parse::<i32>()
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
                                .parse::<i32>()
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
                                .parse::<i32>()
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
                            .parse::<i32>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 2]
                            .parse::<i32>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 3]
                            .parse::<i32>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 4]
                            .parse::<i32>()
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
                            .parse::<i32>()
                            .expect("not valid nr")
                            + self.stack[0])
                            .to_string(),
                        (self.commands.clone()[pointer + 2]
                            .parse::<i32>()
                            .expect("not valid nr")
                            + self.stack[1])
                            .to_string(),
                        (self.commands.clone()[pointer + 3]
                            .parse::<i32>()
                            .expect("not valid nr")
                            + self.stack[0])
                            .to_string(),
                        (self.commands.clone()[pointer + 4]
                            .parse::<i32>()
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
                            .parse::<i32>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 2]
                            .parse::<i32>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 3]
                            .parse::<i32>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 4]
                            .parse::<i32>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 5]
                            .parse::<i32>()
                            .expect("not valid nr"))
                        .to_string(),
                        (self.commands.clone()[pointer + 6]
                            .parse::<i32>()
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
                            .parse::<i32>()
                            .expect("not valid nr")
                            + self.stack[0])
                            .to_string(),
                        (self.commands.clone()[pointer + 2]
                            .parse::<i32>()
                            .expect("not valid nr")
                            + self.stack[1])
                            .to_string(),
                        (self.commands.clone()[pointer + 3]
                            .parse::<i32>()
                            .expect("not valid nr")
                            + self.stack[0])
                            .to_string(),
                        (self.commands.clone()[pointer + 4]
                            .parse::<i32>()
                            .expect("not valid nr")
                            + self.stack[1])
                            .to_string(),
                        (self.commands.clone()[pointer + 5]
                            .parse::<i32>()
                            .expect("not valid nr")
                            + self.stack[0])
                            .to_string(),
                        (self.commands.clone()[pointer + 6]
                            .parse::<i32>()
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

        let points = self.get_cubic_bezier_points(
            (
                self.cartesian_commands[2][1]
                    .parse::<f64>()
                    .expect("not f64"),
                self.cartesian_commands[2][2]
                    .parse::<f64>()
                    .expect("not f64"),
            ),
            (
                self.cartesian_commands[2][3]
                    .parse::<f64>()
                    .expect("not f64"),
                self.cartesian_commands[2][4]
                    .parse::<f64>()
                    .expect("not f64"),
            ),
            (
                self.cartesian_commands[2][5]
                    .parse::<f64>()
                    .expect("not f64"),
                self.cartesian_commands[2][6]
                    .parse::<f64>()
                    .expect("not f64"),
            ),
            (
                self.cartesian_commands[2][7]
                    .parse::<f64>()
                    .expect("not f64"),
                self.cartesian_commands[2][8]
                    .parse::<f64>()
                    .expect("not f64"),
            ),
        );

        println!("{:?}", points);

        // iterate through evert command :
        // - find the origin and target
        // - calculate the length
        // - compare the length to total length of whole SVG to see how many points to get (= nr_points)
        // - devide the height and lenth by the nr_points to find the x and y step
        // - array where you repeatedly add x_step and y_step to get list of points
    }

    fn transform_line_to_cbezier(&mut self, stack: Vec<i32>, end: Vec<String>) -> Vec<String> {
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
                            let mut y_value = command[x].parse::<i32>().expect("not a valid int");

                            // transform to negative value

                            y_value *= -1;

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
    ) -> Vec<(f64, f64)> {
        let mut points = vec![];

        for t in (0..=10).map(|i| i as f64 / 10.0) {
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

        points
    }
}

fn main() {
    let path = "M 20 50 L 100 50 l 50 -30 H 200 h 50 V 100 v 50 Q 300 200 350 150 q -30 -30 -50 -50 C 250 50 200 30 150 150 c 50 30 100 50 150 20";
    let mut pth = Path::init(path);

    pth.get_points();

    println!("{:?}", pth.cartesian_commands);

    //scatter_plot(pth.points);
}
