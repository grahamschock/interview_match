use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    Timestamp: String,
    First_Name: String,
    Last_Name: String,
    Times: String,
}

#[derive(Debug)]
struct Student {
    fname: String,
    lname: String,
    time: Vec<String>,
}
fn example() -> Result<(), Box<dyn Error>> {
    let mut count = 0;
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        let times = record.Times;
        let mut split = times.split(",");
        println!("{}", record.First_Name);
        // for s in split {
        //     println!("{}", s);
        // }
        let s1: Vec<String> = split
            .map(|val| val.parse().unwrap())
            .collect();
        let student = Student {
            fname: record.First_Name,
            lname: record.Last_Name,
            time: s1,
        };
        println!("{:?}", student);
        // println!("{}", times);
        count = count + 1;
    }
    println!("{}", count);
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
