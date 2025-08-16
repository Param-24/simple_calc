use std::{env, 
    io::{self, Write}};

fn main(){
    let nums:Vec<i32> = env::args()
                        .skip(1)
                        .map(|arg| arg.parse().expect("Enter numbers as command line arguments."))
                        .collect();
    print!("Enter the operation you want to perform on the given values: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut op = String::new();
    io::stdin()
        .read_line(&mut op)
        .expect("Please enter operation you want to perform on the numbers");
    let op = op.trim();

    let result:i32 = match op {
        "+" | "add" | "sum" => nums.iter().sum(),
        "-" | "sub" | "diff" => {
            let (first_elm, other_elms) = nums.split_first().unwrap();
            other_elms.iter().fold(*first_elm, |acc, &x| acc - x)
        },
        "*" | "mult" | "prod" => nums.iter().product(),
        "/" | "div" => {
            let (first_elm, other_elms) = nums.split_first().unwrap();
            if other_elms.contains(&0){
                eprintln!("Error!! Division by zero.");
                return;
            }
            other_elms.iter().fold(*first_elm, |acc, &x| acc / x)
        },
        _ => return,
    };
    println!("The result for the operation is: {result}");

}