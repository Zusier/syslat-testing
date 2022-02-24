use serde_json::Value;
use std::{
    collections::{HashMap},
    fs::File,
    io::{BufReader, Write},
};

fn main() {
    //let file = File::open("test_data.json").unwrap();
    let file_to_open = std::env::args().nth(1).expect("Could not open input json!");
    let file = File::open(file_to_open.clone()).unwrap();
    // input file with file extension replaced with .svg
    let svg = format!("{}{}", file_to_open.trim_end_matches(".json"), ".svg");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).unwrap();

    // Todo: use counts/total from json
    let mut results_vec: Vec<u64> = Vec::new();

    let results_count = json["AggregateData"].as_object().unwrap().get("SysLatTestCount").unwrap().as_u64().unwrap();
    let results_total = json["AggregateData"].as_object().unwrap().get("SystemLatencyTotal").unwrap().as_u64().unwrap();
    //const results_count: usize = results_count as usize;

    for entry in json["SysLatData"].as_object().unwrap()["SysLatResults"].as_array().unwrap() {
        //println!("{:?}", entry.as_u64().unwrap());
        results_vec.push(entry.as_u64().unwrap());
    }
    results_vec.sort();
    //results_vec.reverse();
    // calculate 1% percentile lows
    {
        let mut total = 0;
        for result in results_vec.iter() {
            total += *result;
            if total >= results_total / 100 {
                //println!("1% Low: {}ms", *result);
                break;
            }
        }
    }
    //println!("Average: {}ms", results_total / results_count);
    // Easily an outlier, perhaps try and remove if above average by a multiplier
    //println!("Maximum: {}ms", results_vec[results_vec.len() - 1]);

    // build data for plot
    let mut data: HashMap<i128, i128> = HashMap::new();
    let mut count = 0;
    for entry in json["SysLatData"].as_object().unwrap()["SysLatResults"].as_array().unwrap() {
        count += 1;
        data.insert(count, entry.as_i64().unwrap().try_into().unwrap());
    }

    // create a plot
    let data = poloto::data::<i128, i128>().scatter("", data).ymarker(0).xmarker(0).build();

    // edit stepping
    let (xtick, xtick_fmt) = poloto::steps(data.boundx(), (0..).step_by(500));
    let (ytick, ytick_fmt) = poloto::steps(data.boundy(), (0..).step_by(15));

    let mut plotter = data.plot_with(
        xtick,
        ytick,
        poloto::plot_fmt("SysLat Test Graph", "Seconds Elapsed", "Latency (in milliseconds)", xtick_fmt, ytick_fmt),
    );
    let mut file = File::create(svg.clone()).unwrap();
    write!(
        file,
        "{}<style>{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        poloto::disp(|w| plotter.render(w)),
        poloto::simple_theme::SVG_END,
    ).unwrap();
}
