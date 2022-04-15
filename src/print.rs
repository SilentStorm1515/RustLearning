//1. PRINTING TO THE CONSOLE

//pub keyword always creates a public function/variable
//fn keyword initializes a function
pub fn run() {
    //Print to console
    //use println! function to print a line
    println!("Hello from the print.rs file!");

    //cannot print ints, use a placeholder ("{}") and then the number to insert into a string
    println!("Number: {}", 5);

    //each placeholder assigns to the next available inserted string
    println!("{} is from {}", "Dhruv", "Calgary");

    //positional arguments can be used to utilize the same variable in multiple places, 0-indexed
    println!("{0} is from {1} and {0} likes to {2}", "Dhruv", "Calgary", "code");

    //named arguments can also be used
    println!("{name} likes to play {activity}", name = "Dhruv", activity = "video games");

    //placeholder traits to write numbers in different forms
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    //placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //basic math
    println!("10 + 10 = {}", 10 + 10);
}