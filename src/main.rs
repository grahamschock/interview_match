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
    isStudent: i32,
}


#[derive(Debug)]
struct Person {
    fname: String,
    lname: String,
    time: Vec<String>,
    is_assigned: i32,
}


fn example() -> Result<(), Box<dyn Error>> {
    let mut count = 0;
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut students = vec![];
    let mut instructors = vec![];
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        let times = record.Times;
        let mut split = times.split(",");
        // for s in split {
        //     println!("{}", s);
        // }
        let s1: Vec<String> = split
            .map(|val| val.parse().unwrap())
            .collect();

        let student = Person {
            fname: record.First_Name,
            lname: record.Last_Name,
            time: s1,
            is_assigned: 0,
        };

        if record.isStudent == 1 {
            students.push(student);
            count+=1;
        } else {
            instructors.push(student);
        }
    }
    students.sort_by(|a, b| a.time.len().cmp(&b.time.len()));
    instructors.sort_by(|a,b| a.time.len().cmp(&b.time.len()));


    for instructor in instructors.iter() {
        let mut assignments = 0;
        println!("INSTRUCTOR: {}------", instructor.fname);
        for time1 in instructor.time.iter() {
            let index = find_student(time1.to_string(), &students);
            if index != None && assignments <= (((count - 1) / 3)){
                println!("STUDENT: {}", students[index.unwrap()].fname);
                println!("TIME: {}", time1);
                assignments += 1;
                students.remove(index.unwrap());
            }
        }
    }
    println!("-------STUDENTS LEFT-----");
    println!("{:?}", students);
    // println!("-------INSTRUCTORS--------");
    // println!("{:?}", instructors);
    println!("NUM STUDENTS: {}", count);
    Ok(())
}

fn find_student(time: String, students: &Vec<Person>) -> Option<usize> {
    let time1 = time.trim_start().to_string();
    let mut count = 0;
    for student in students.iter() {
        let index = student.time.iter().position(|r| r.trim_start().eq(&time1));
        if index != None {
            // println!("STUDENT: {}", student.fname);
            // println!("TIME: {}", student.time[index.unwrap()]);
            return Some(count);
        }
        count = count + 1;
    }
    None
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
