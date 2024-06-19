use array::to_num_sum::Solution;
use array::two_array_mid::Solution1;

mod array;

fn main() {
    //1
    let arr1 = vec![1,2,3];
    let target = 6;
    // let a = Solution::create(arr1, target);
    //2
    let arr2 = vec![4,7];
    let b  = Solution1::create(arr1, arr2);
    println!("Hello, world1!{:?}",b.find_mid());
}
