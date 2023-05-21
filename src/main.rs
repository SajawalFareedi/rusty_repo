#![allow(unused)]

use core::num;
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader, Write, ErrorKind};
use rand::Rng;
use std::cmp::Ordering;

// ------------------------------ //
// expect + io
// ----------------------------- //
fn expect() {
    let mut name = String::new();
    let greeting = "Nice to meet ya!";
    println!("Please type your name: ");
    io::stdin().read_line(&mut name).expect("Didn't recieved input");
    println!("Hello {}! {}", name.trim_end(), greeting);
}

// ------------------------------ //
// shadowing + constants
// ----------------------------- //
fn shadowing() { 
    // Shadowing is a technique in which a variable declared within a certain scope has the same name as a variable declared in an outer scope //
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "19";
    let mut age: u32 = age.trim().parse().expect("Age was not a number! Couldn't parse it...");
    age += 1;
    println!("I'm {} and I've ${}", age, ONE_MIL);
}

// ------------------------------ //
// data-types
// ----------------------------- //
fn data_types() {
    // Floats + Ints + Unsigned Ints //
    println!("u32: {}", u32::MAX);
    println!("u64: {}", u64::MAX);
    println!("u128: {}", u128::MAX);
    println!("usize: {}", usize::MAX); // usize means an unsigned integer whose len depends on its usage // usize has same MAX as u64 for x86_x64, and u32 for x32 arch

    println!("f32: {}", f32::MAX);
    println!("f64: {}", f64::MAX);

    println!("i32: {}", i32::MAX);
    println!("i64: {}", i64::MAX);
    println!("i128: {}", i128::MAX);
    println!("isize: {}", isize::MAX); // isize means an signed integer whose len depends on its usage // isize has same MAX as i64 for x86_x64, and i32 for x32 arch

    // bools + add '_' before var name - if you want to have a var but don't want to use and don't want warning for it either //
    let _is_true = true;
    let _is_false = false;

    // Char // Use single quotes for 'chars' and double for "strings"
    let _my_grade = 'A';
}

// ------------------------------ //
// mathematics
// ----------------------------- //
fn maths() {
    let num_1: f32 = 1.111111111111111;
    let num_2: f64 = 1.111111111111111;

    // Floats precesion //
    println!("f32 upto 7 digits: {}", num_1 + 0.111111111111111);
    println!("f64 upto 14 digits: {}", num_2 + 0.111111111111111);

    // Basic operators //
    let num_3 = 5;
    let num_4: u32 = 4;

    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    // Generate random number //
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("Random num: {}", random_num);
}

// ------------------------------ //
// if_else + ternary + match
// ----------------------------- //
fn if_else() {
    let age = 19;
    let voting_age = 18;

    // If else //
    if (age >= 1 && age <= 18) {
        println!("Important Birthday");
    } else if (age == 21) || (age == 50) {
        println!("Important Birthday");
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an Important Birthday");
    }

    // ternary operator //
    let can_vote = if age >= 18 {true} else {false};
    println!("Can vote: {}", can_vote);

    // match //
    match age {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an Important Birthday"),
    };

    match age.cmp(&voting_age) {
        Ordering::Less => println!("You can't vote!"),
        Ordering::Greater => println!("You can vote!"),
        Ordering::Equal => println!("You just earned the right to vote!"),
    };

}

// -------------------------------- //
// Loops + Arrays + Tuples
// -------------------------------- //
fn loop_arr() {
    // Arrays // First one is with fixed length of 6 // Second one has dynamic length //
    let arr_1: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let arr_2 = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'];

    println!("First element of arr_1: {}", arr_1[0]);
    println!("Length of arr_2: {}", arr_2.len());

    // While Loop //
    let mut idx = 0;

    while idx < arr_1.len() {
        println!("arr_1[{}] = {}", idx, arr_1[idx]);
        idx += 1;
    }

    // For Loop //
    for v in arr_2.iter() {
        println!("Val: {}", v);
    }

    // Tuples // You've to define the types respective to the values in the tuple //
    let tup: (u8, String, f32) = (19, "Sajawal".to_string(), 9_000_000_000_000_000_000_000.00);
    println!("\nName: {}", tup.1);

    // Tuple destructring - Just like in python or in javascript (object destructring)
    let (v1, v2, v3) = tup;
    println!("Age: {} \nWealth: ${}\n", v1, v3);

}

// -------------------------------- //
// Strings
// -------------------------------- //
fn strings() {
    // Creating a String
    let st1 = String::from("Hello World!");
    println!("{}", st1);

            // OR //

    let mut st2 = String::new();
    println!("{}", st2); // empty string

    // Pushing in string
    st2.push('A');
    st2.push_str(" Word");

    // Looping string
    for w in st2.split_whitespace() {
        println!("{}", w);
    }

    // Replacing in string
    let st3 = st2.replace("A", "Another");
    println!("{}", st3);

    // Creating a vector from string chars
    let st4 = String::from("x g r y j h h g f e f v x d");
    let mut st5: Vec<char> = st4.chars().collect();

    // Sorting and removing duplicate
    st5.sort();
    st5.dedup();

    // Looping over vector //
    for c in st5.iter() {
        println!("{}", c);
    }

    let st6 = "Random String";
    let mut st7 = st6.to_string();

    // String slicing
    let st8 = &st7[0..6];
    println!("{}", st8);

    // Clearing string
    st7.clear();
    println!("{}", st7);

    // String combining
    let st9 = String::from("Just Some");
    let st10 = String::from(" Random Words");
    let st11 = st9 + &st10; // One thing to note here is that the '&' is used for referencing to that string. We didn't put it with st9 so after this line gets executed st9 will not exists because its st11 now with st10 combined. You must not add '&' sign to the first string. You can add it to the rest. In this case we added to st10 but didn't added it to the first string which is st9. //
    println!("{}", st11);

}

// ------------------------------ //
// casting
// ----------------------------- //
fn casting() {
    // Casting means converting from one data type to other
    // Like - u8 to u32

    // We always use 'as' to convert

    // u8 -> u32
    let u8_1: u8 = 5;
    let u8_2: u8 = 4;
    let u32_0: u32 = (u8_1 as u32) + (u8_2 as u32);

    let dec = 65.4321_f32;
    // f32 -> i32
    let int = dec as i32;
    // i32 -> u8
    let int2 = int as u8;
    // u8 -> char
    let char  = int2 as char; // Note: You can only convert u8 to char //
}

// ----------------------------- //
// Enumerators
// ----------------------------- //
fn enums() {
    // Enums are collections of variants
    // Each variant is a case
    // Each case is assigned a value

    // First define all of the variants
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    // Then you can define functions for those variants
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    // Declare a variable with data type set to that enum 
    let day: Day = Day::Tuesday;
    
    // Use the function which are in impl
    if day.is_weekend() {
        println!("It's a weekend!");
    } else {
        println!("It's a weekday!");
    }
}


// ------------------------------ //
// main is necessary in rust
// ----------------------------- //
fn main() { 
    // Calling the required function //
    // expect();
    // shadowing();
    // data_types();
    // maths();
    // if_else();
    // loop_arr();
    // strings();
    // casting();
    enums();
}
