#![allow(unused)]
use std::io;
use std::fs::File;
use std::io::{Write, ErrorKind, BufReader, BufRead};
use std::cmp::Ordering;
use std::ops::Add;

fn main() {
    trait Classes {
    fn attendance(total: f32, attended: f32) -> f32 {
        (attended / total) * 100.0
    }
    }
    struct Student {
        rollno: i32,
    }
}
