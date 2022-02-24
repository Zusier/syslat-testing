use std::{fs::File, io::{Read, self, BufReader}};

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

    let json: Value = serde_json::from_reader(reader).unwrap();

    for entry in json["SysLatData"].as_object().unwrap()["SysLatResults"].as_array().unwrap() {
        //println!("{:?}", entry.as_u64().unwrap());
        results_total += entry.as_u64().unwrap();
        results_count += 1;
    }

    println!("Average Latency: {}ms", results_total / results_count);
}