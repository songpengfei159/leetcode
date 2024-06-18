pub struct Solution{
    target: i32,
    arr: Vec<i32>
}

impl Solution {
    pub fn create(arr: Vec<i32>,target: i32) -> Self {
        Self{
            target,
            arr
        }
    }
    pub fn find_target(&self) -> Vec<i32>{
        let mut tmp = Vec::<i32>::new();
        let mut end = self.arr.len()-1;
        let mut begin = 0;
        loop {
            if self.arr[begin]+self.arr[end]>self.target {
                end -= 1;
            } else if self.arr[begin]+self.arr[end]<self.target{
                begin += 1;
            }else{
                tmp.push(begin as i32);
                tmp.push(end as i32);
                break;
            }
            if begin == end {
                break;
            }
        }
        tmp
    }
}