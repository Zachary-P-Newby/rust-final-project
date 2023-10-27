#![allow(unused)]
use std::io;
use std::io::{Write, BufReader, BufRead, BufWriter};
use std::io::Error;
use std::fs::File;

fn main() {
    let mut path = String::new();

    print!("Enter a file path: ");
    io::stdin().read_line( &mut path);

    println!("{}",path);
}
