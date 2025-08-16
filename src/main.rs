use std::{env, 
    io::{self, Write}};

fn main(){
    let nums:Vec<u16> = env::args()
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

    let mut result:u16 = 0;
    match op {
        "+" | "add" | "sum" => {
            for i in nums {
                result+=i;
            }
        },
        "-" | "sub" | "diff" => {
            for i in nums {
                result-=i;
            }
        },
        "*" | "mult" | "prod" => {
            for i in nums {
                result*=i;
            }
        },
        "/" | "div" => {
            for i in nums {
                result/=i;
            }
        },
        _ => eprint!("Enter a valid operation")
    };
    println!("The result for the operation is: {result}");

}