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
    let results_count = json["AggregateData"].as_object().unwrap().get("SysLatTestCount").unwrap().as_u64().unwrap();
    // Gather total ms latency from data points
    let results_total = json["AggregateData"].as_object().unwrap().get("SystemLatencyTotal").unwrap().as_u64().unwrap();

    for entry in json["SysLatData"].as_object().unwrap()["SysLatResults"].as_array().unwrap() {
        results_vec.push(entry.as_u64().unwrap());
    }
    results_vec.sort_unstable();

    // calculate 1% percentile lows
    {
        let mut total = 0;
        for result in results_vec.iter() {
            total += *result;
            if total >= results_total / 100 {
                println!("1% Low: {}ms", *result);
                break;
            }
        }
    }
    println!("Average: {}ms", results_total / results_count);
    // Easily an outlier, perhaps try and remove if above average by a multiplier
    println!("Maximum: {}ms", results_vec[results_vec.len() - 1]);

    // build data for plot
    let mut data: HashMap<i128, i128> = HashMap::new();
    let mut count = 0;
    for entry in json["SysLatData"].as_object().unwrap()["SysLatResults"].as_array().unwrap() {
        count += 1;
        data.insert(count, entry.as_i64().unwrap().try_into().unwrap());
    }

    // create a plot
    let data = poloto::data::<i128, i128>().scatter("", data).ymarker(0).xmarker(0).build();

    // edit scatter plot stepping
    let (xtick, xtick_fmt) = poloto::steps(data.boundx(), (0..).step_by(500));
    let (ytick, ytick_fmt) = poloto::steps(data.boundy(), (0..).step_by(15));

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
