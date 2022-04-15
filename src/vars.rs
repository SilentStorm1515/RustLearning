//variables can hold primitive data or references to data
//variables are immutable (not editable) by default
//Rust is block-scoped

pub fn run() {
    //use "let" to create variables
    let name = "Dhruv";

    //"mut" keyword makes variable mutable, immutable by default
    let mut age = 16;
    println!("My name is {} and I am {}", name, age);

    age = 17;
    println!("My name is {} and I am {}", name, age);

    //define constant using "const" keyword
    //i32 means 32-bit integer (must define type when using const)
    //note: i only goes up to i128
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //assign multiple vars at once
    let ( my_name, my_age ) = ("Dhruv", 16);
    println!("{} is {} years old", my_name, my_age);

}