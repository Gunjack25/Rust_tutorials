#![allow(unused)]

use std::io; //Adding package input output 
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

//Same as C++
fn make_age_as_28(Age:&mut u32)
{
    *Age = 28;
}
fn print_age(Age:&u32)
{
    println!("Age is {}", Age);
}

fn main() {
    //! stands for macro
    println!("What is your name?");
    //mut values can be changed 
    let mut name: String = String::new();
    io::stdin().read_line(&mut name) 
    .expect("Didn't receive Input");
    //If error then the message is displayed Ok, Error
    //read_line returns Result which has enum value Ok, Error
    println!("Hello, {} ", name.trim_end());
    const PI : f32 = 3.14;
    let age: &str = "25";
    let mut age: u32 = age.trim().parse()
    .expect("Age not declared");
    make_age_as_28(&mut age);
    print_age(&age);
}
