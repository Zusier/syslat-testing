// Todo:
// Split common stuff into functions (not much, which is why it's all under main)
use serde_json::Value;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Write},
};

fn main() {
    let file_to_open = std::env::args().nth(1).expect("Could not find any argument!");

    let file = File::open(file_to_open.clone()).expect("Could not open log file! (is the file path correct?)");

    // input file with file extension replaced with .svg
    let svg = format!("{}{}", file_to_open.trim_end_matches(".json"), ".svg");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).unwrap();

    let mut results_vec: Vec<u64> = Vec::new();

    // Gather how many data points were collected
    let mut results_count: usize = 0;
    // Gather total ms latency from data points
    let mut results_total: u64 = 0;

    let mut data: HashMap<i128, i128> = HashMap::new(); // no unsigned ints implemented in poloto crate :(
    for entry in json["SysLatData"].as_object().unwrap()["SysLatResults"].as_array().unwrap() {
        let e_u64: u64 = entry.as_u64().unwrap();
        #[allow(clippy::if_same_then_else)]
        // Have to process first two entries for the outlier removal to work
        // this is actually faster compared to running the first two entries separately and not having if statements
        if results_vec.len() <= 2 {
            results_count += 1; // increment count for plot timeline
            results_total += e_u64; // add to total for plot timeline
            data.insert(results_count as i128, e_u64 as i128); // build data for plot
            results_vec.push(e_u64); // going to sort vector for lows, averages, and highs
        } else if e_u64 <= (results_vec[results_count - 1]) * 4 { // if data point is 4 times higher than last data point, do not add
            results_count += 1;
            results_total += e_u64;
            data.insert(results_count as i128, e_u64 as i128);
            results_vec.push(e_u64);
        }
    }
    results_vec.sort_unstable();

    // calculate 1% percentile lows
    {
        let mut total: u64 = 0;
        for result in results_vec.iter() {
            total += result;
            if total >= results_total / 100 {
                println!("1% Low: {}ms", result);
                break;
            }
        }
    }
    println!("Average: {}ms", results_total / results_count as u64);
    // Easily an outlier, perhaps try and remove if above average by a multiplier
    println!("Maximum: {}ms", results_vec[results_vec.len() - 1]);

    // create a plot
    let data = poloto::data::<i128, i128>().scatter("", data).ymarker(5).xmarker(0).build();

    // edit scatter plot stepping
    let (xtick, xtick_fmt) = poloto::steps(data.boundx(), (0..).step_by(500));
    let (ytick, ytick_fmt) = poloto::steps(data.boundy(), (5..).step_by((results_vec[results_vec.len() - 1] / 10).try_into().unwrap())); // step by 10% of max

    let mut plotter = data.plot_with(
        xtick,
        ytick,
        poloto::plot_fmt("SysLat Test Graph", "Seconds Elapsed", "Latency (in milliseconds)", xtick_fmt, ytick_fmt),
    );
    let mut file = File::create(svg).expect("Could not create svg!");
    write!(
        file,
        "{}<style>{}.poloto_scatter{{stroke-width:3}}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        //".poloto_scatter{stroke-width:3}",
        poloto::disp(|w| plotter.render(w)),
        poloto::simple_theme::SVG_END,
    )
    .expect("Failed to write svg!");
}
