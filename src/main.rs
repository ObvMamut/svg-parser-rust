use plotly::Layout;
use plotly::Plot;
use plotly::Scatter;
use rand_distr::num_traits::Pow;

// Scatter Plots
fn scatter_plot(points: Vec<Vec<f32>>) {
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
    points: Vec<Vec<f32>>,
    stack: Vec<f32>,
    n: i32,
}

impl<'a> Path<'a> {
    fn init(path: &'a str) -> Self {
        Path {
            path,
            commands: Self::dissect_path(path),
            points: vec![],
            stack: vec![],
            n: 15,
        }
    }

    fn calculate_total_length(&mut self) -> f32 {
        // iterate through every command (Line or Bezier Curve)
        // calculate it's length
        // add it to the total
        // return the total

        let mut total: f32 = 0.0;

        for x in 0..self.commands.len() {
            match self.commands[x as usize].as_str() {
                _ => {}
            }
        }

        return total;
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

    fn synthesize_command(&mut self) -> Vec<String> {

        let mut stack = vec![];

        let mut last_command = vec![];

        for e in self.commands.len() {
            match self.commands[e].as_str() {
                _ => {
                }
            }
        }


    }

    fn get_points(&mut self) {

        // synthesize (remove every relative term)
        // transform every command except M to a bezier curve
        // iterate through evert command :
        // - find the origin and target
        // - calculate the length
        // - compare the length to total length of whole SVG to see how many points to get (= nr_points)
        // - devide the height and lenth by the nr_points to find the x and y step
        // - array where you repeatedly add x_step and y_step to get list of points

        let mut pointer = 0;

        for element in self.commands.clone() {
            match element.as_str() {

                _ => {}
            }
            pointer += 1;
        }
    }



fn main() {
    let path = "M 175 200 l 150 0";
    let mut pth = Path::init(path);

    pth.get_points();

    // println!("{:?}", pth.points);

    let total = pth.calculate_total_length();

    println!("{:?}", total);

    scatter_plot(pth.points);
}
