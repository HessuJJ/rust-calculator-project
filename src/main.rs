fn main() {
    //Loop until user chooses to exit
    loop {
        // Display menu
        println!("-- Calculator -- \n 1. Add \n 2. Subtract \n 3. Multiply \n 4. Divide \n 5. Exit \n choose an option (1-5): ");
        // Read user input
        let i: i32 = read_input();
        // Match user input to corresponding function
        if i == 1 {
            let result = add();
            println!("Result: {result}");
        } else if i == 2 {
            let result = subtract();
            println!("Result: {result}");
        } else if i == 3 {
            let result = multiply();
            println!("Result: {result}");
        } else if i == 4 {
            let result = divide();
            println!("Result: {result}");
        } else if i == 5 {
            exit();
        } else {
            error();
        }
        println!("\n");
    }
}
// Function to add two numbers
fn add() -> i32 {
    println!("Enter first number: ");
    let a: i32 = read_input();
    println!("Enter second number: ");
    let b: i32 = read_input();
    a + b
}
// Function to subtract two numbers
fn subtract() -> i32 {
    println!("Enter first number: ");
    let a: i32 = read_input();
    println!("Enter second number: ");
    let b: i32 = read_input();
    a - b
}
// Function to multiply two numbers
fn multiply() -> i32 {
    println!("Enter first number: ");
    let a: i32 = read_input();
    println!("Enter second number: ");
    let b: i32 = read_input();
    a * b
}
// Function to divide two numbers
fn divide() -> i32 {
    println!("Enter first number: ");
    let a: i32 = read_input();
    println!("Enter second number: ");
    let b: i32 = read_input();
    if b == 0 {
        println!("Cannot divide by zero");
        0
    } else {
        a / b
    }
}
// Function to exit the program
fn exit() {
    println!("Exiting...");
    std::process::exit(0);
}
// Function to display error message
fn error() {
    println!("Invalid input, please enter a number between 1 and 5.");
}
// Function to read user input
fn read_input() -> i32 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    // Match the input to an integer, if it fails, ask for input again
    match input.trim().parse::<i32>() {
        // If the input is a valid integer, return it
        Ok(num) => num,
        // If the input is not a valid integer, ask for input again
        Err(_) => {
            println!("Invalid input, please enter a number.");
            // Call the function again to get valid input
            read_input()
        }
    }
}
