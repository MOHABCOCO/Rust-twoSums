fn main() {
    let nums = vec![1,2,3];
    println!("{:?}", two_sum(&nums, 3));
}

fn two_sum(nums : &Vec<i32>, target: i32) -> Vec<i32>{
    for x in 0..nums.len(){
        for y in x+1..nums.len(){
            if nums[x] + nums[y] == target{
                return vec![x as i32, y as i32];
            }
        }
    }
    return vec![];
}