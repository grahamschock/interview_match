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
        println!("{:?}", record);
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
