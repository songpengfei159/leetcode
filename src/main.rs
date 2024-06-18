use array::to_num_sum::Solution;

mod array;

fn main() {
    let arr = vec![1,2,3];
    let target = 6;
    let a :Solution = Solution::create(arr, target);
    println!("Hello, world!{:?}",a.find_target());
}
