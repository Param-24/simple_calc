use std::io::{self, Write};

pub fn get_nums() -> Option<Vec<i32>>{
    print!("Enter the numbers you want to perform calculation on, separated by spaces: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut num_input = String::new();
        io::stdin()
        .read_line(&mut num_input)
        .expect("Please enter the numbers.");
    let nums:Vec<i32> = num_input
                        .trim()
                        .split_whitespace()
                        .map(|x| x.parse().expect("Numbers are expected in input."))
                        .collect();
    if nums.len()<2{
        eprintln!("Enter atleast two operands.");
        return None;
    }
    return Some(nums);
}



pub fn get_op() -> String{
    print!("Enter the operation you want to perform on the given values: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut op = String::new();
    io::stdin()
        .read_line(&mut op)
        .expect("Please enter operation you want to perform on the numbers");
    let op = op.trim();
    return op.to_string();
}