pub fn add(nums:Vec<i32>) -> i32{
    return nums.iter().sum();
}

pub fn diff(nums:Vec<i32>) -> i32{
    let (first_elm, other_elms) = nums.split_first().unwrap();
    other_elms.iter().fold(*first_elm, |acc, &x| acc - x)
}

pub fn prod(nums:Vec<i32>) -> i32{
    return nums.iter().product()
}

pub fn div(nums:Vec<i32>) -> i32{
    let (first_elm, other_elms) = nums.split_first().unwrap();
    if other_elms.contains(&0){
        eprintln!("Error!! Division by zero.");
        return 0;
    }
    other_elms.iter().fold(*first_elm, |acc, &x| acc / x)
}