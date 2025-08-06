pub fn sum(nums: Vec<i32>) -> i32 {
    let mut x = 0;
    for i in nums{
        x += i;
    }
    x
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    let mut v :Vec<u32> =Vec::new();
    let mut x =0;
    while x < n{
        v.push(i);
        x+=1;
    }
    v
}
