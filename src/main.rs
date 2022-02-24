use poloto::{prelude::*, plotnum::{HasDefaultTicks, PlotNum}};
use serde_json::Value;
use std::{fs::File, io::{BufReader, self}, collections::{hash_map::Entry, HashMap}};

fn main() {
    // .json found at ./data.json
    let file = File::open("test_data.json").unwrap();
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

    let mut data: HashMap<i128, i128> = HashMap::new();
    //let data: [(u64, u64); json["AggregateData"].as_object().unwrap().get("SysLatTestCount").unwrap().as_u64().unwrap()];
    let mut count = 0;
    for entry in json["SysLatData"].as_object().unwrap()["SysLatResults"].as_array().unwrap() {
        count += 1;
        data.insert(count, entry.as_i64().unwrap().try_into().unwrap());
    }

    //let data = data.iter().map(|(k, v| (v.0, k.clone())).collect::<Vec<_>>();
    //let data = data.iter().map(|(k, v)| (v, k.clone())).collect::<Vec<_>>();
    let data = poloto::data::<i128, i128>()
        .histogram("Latency", data)
        .ymarker(0)
        .xmarker(0)
        .build();
    let mut plotter = data.plot(
        "SysLat Test Graph",
        "Seconds elapsed",
        "Latency (in milliseconds)",
    );

    print!("{}", poloto::disp(|w| plotter.simple_theme_dark(w)));
    // write to svg file
    //io::write::file("test.svg", poloto::disp(|w| plotter.simple_theme_dark(w)));
}
