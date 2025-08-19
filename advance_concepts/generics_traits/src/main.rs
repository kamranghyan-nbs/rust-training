pub fn sum_of_elements(nums: &[i32]) -> i32 {
    let mut sum = nums[0];
    for i in 1..nums.len() {
        sum += nums[i];
    }
    sum
}

pub fn sum_of_elements_generic<T: std::ops::AddAssign + Copy>(nums: &[T]) -> T {
    let mut sum = nums[0];
    for i in 1..nums.len() {
        sum += nums[i];
    }
    sum
}

fn main() {
    let my_nums = [1,2,3,4,5];
    let your_nums = [6,7,8,9,2];

    let my_sum = sum_of_elements(&my_nums);
    let  your_sum = sum_of_elements(&your_nums);

    print!("My Sum : {}", my_sum);
    print!("");
    print!("Your Sum : {}", your_sum);

    let my_i64_nums = [2,4,6,8,2];
    let my_i64_sum = sum_of_elements_generic(&my_i64_nums);
    print!("My i64 Sum : {}", my_i64_sum);

    // let v1 = vec![1,2,3];
    // let v2 = vec![4,5,6];

    // let v3 = v1 + v2; 

}
