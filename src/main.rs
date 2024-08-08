#![allow(unused)]
use std::io;
use std::fs::File;
use std::io::{Write, ErrorKind, BufReader, BufRead};
use std::cmp::Ordering;
use std::ops::Add;

fn main() {
    trait Classes {
        fn new(name: String, rollno: i32, total: f32, attended: f32) -> Self;
        fn attendance(&self) -> f32;
    }
    struct Student {
        name: String,
        rollno: i32,
        total: f32,
        attended: f32
    }

    impl Classes for Student {
        fn new(name: String, rollno: i32, total: f32, attended: f32) -> Student{
            Student{name, rollno, total, attended}
        }
        fn attendance(&self) -> f32 {
            (self.attended / self.total) * 100.0
        }
    }

    let path = std::path::Path::new("attendance.txt");
    let path = std::path::Path::new("attendance.txt");

    let stud2 = <Student as Classes>::new("Karttikeya".to_string(), 102313034, 35.0, 19.0);
    // println!("Attendance is {}", stud2.attendance());

}
