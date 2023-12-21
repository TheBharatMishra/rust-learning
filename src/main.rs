#[allow(unused)]

use std::io;
// use rand::Rng;
// use std::io::{BufReader,BufRead,ErrorKind,Write};
// use std::fs::File;
// use std::cmp::Ordering;

fn main() {
    // println!("What is yout name?");
    // let mut someName: String= String::new(); 
    // someName.push('c');
    // let greeting: &str="Nice to meet you";
    // io::stdin().read_line( &mut someName)
    // .expect("We didnt recieve any input");
    // println!("Hello, {}! ,{}",someName.trim_end(),greeting);
    const ONE_MIL :u32=1_000_000;
    // const PI:f32= 3.141592;
    let age: &str="47";
    let mut age : u32 = age.trim().parse()
        .expect("Age wasnt assigned a number");

    age = age+1;
    println!("I am {} and I want {}.",age,ONE_MIL);
}