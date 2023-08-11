
fn calculate_sum_square(nums:&[i32]) -> i32 {
    let mut total = 0;
    for &num in nums {
        total += num * num
    }
    total
}

fn main() {
    let nums = [1, 2, 3, 4, 5];
    let total = calculate_sum_square(&nums);
    println!("The total is {:?}", total);
}
