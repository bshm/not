use std::env;
use std::process::{Command, exit};

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure the program is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <program> [args...]", args[0]);
        exit(1);
    }

    // Extract program name and its parameters
    let program = &args[1];
    let parameters: Vec<&str> = args.iter().skip(2).map(|s| s.as_str()).collect();

    // Execute the program
    let mut child = Command::new(program)
        .args(parameters)
        .spawn()
        .expect("Failed to start the process");

    // Wait for the program to finish
    let status = child.wait().expect("Failed to wait for the process");

    // Negate the exit code
    let negated_exit_code = if status.success() {
        1
    } else {
        0
    };

    // Exit with the negated exit code
    exit(negated_exit_code);
}
