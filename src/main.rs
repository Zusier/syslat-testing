use std::{
    fs::File,
    io::{BufReader},
};
use serde_json::Value;

fn main() {
    // .json found at ./data.json
    let file = File::open("test_data.json").unwrap();
    let reader = BufReader::new(file);

    // Todo: use counts/total from json
    // total ms in the file
    let mut results_total: u64 = 0;
    // how many data points are in the file
    let mut results_count: u64 = 0;
    let mut results_vec: Vec<u64> = Vec::new();

    let json: Value = serde_json::from_reader(reader).unwrap();

    for entry in json["SysLatData"].as_object().unwrap()["SysLatResults"]
        .as_array()
        .unwrap()
    {
        //println!("{:?}", entry.as_u64().unwrap());
        results_total += entry.as_u64().unwrap();
        results_count += 1;
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
                println!("1% Low: {}ms", *result);
                break;
            }
        }
    }
    println!("Average: {}ms", results_total / results_count);
    // Easily an outlier, perhaps try and remove if above average by a multiplier
    println!("Maximum: {}ms", results_vec[results_vec.len() - 1]);
}
