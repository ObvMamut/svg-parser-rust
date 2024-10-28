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
    n: i32,
}

impl<'a> Path<'a> {
    fn init(path: &'a str) -> Self {
        Path {
            path,
            commands: Self::dissect_path(path),
            points: vec![],
            stack: vec![],
            synth_commands: vec![],
            n: 10,
        }
    }

    fn dissect_path(path: &str) -> Vec<String> {
        let mut result = vec![];
        let mut input = path;

        let mut current = String::new();

        for ch in input.chars() {
            if ch.is_alphabetic() {
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
                result.push(ch.to_string());
            } else if ch.is_numeric() {
                current.push(ch);
            } else if ch.is_whitespace() {
                if !current.is_empty() {
                    result.push(current.clone());
                    current.clear();
                }
            }
        }

        if !current.is_empty() {
            result.push(current);
        }

        println!("{:?}", result);

        return result;
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

                    // update stack

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
                _ => {
                    pointer += 1;
                }
            }
        }

        println!("{:?}", self.synth_commands)
    }

    fn get_points(&mut self) {
        // synthesize transform every command except M to a bezier curve

        self.synthesize();

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
}

fn main() {
    let path = "M 0 0 L 100 100 v 10";
    let mut pth = Path::init(path);

    pth.get_points();

    //scatter_plot(pth.points);
}
