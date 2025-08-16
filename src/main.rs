mod calc;
mod input;

fn main(){

    let nums: Vec<i32> = input::get_nums().unwrap();

    let op = input::get_op();

    let result:i32 = match op.as_str() {
        "+" | "add" | "sum" => calc::add(nums),
        "-" | "sub" | "diff" => calc::diff(nums),
        "*" | "mult" | "prod" => calc::prod(nums),
        "/" | "div" => calc::div(nums),
        _ => return,
    };
    println!("The result for the operation is: {result}");

}