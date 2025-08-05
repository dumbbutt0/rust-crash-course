#![allow(unused)]
pub fn mul(x: u32, y: u32) -> u32 {
    return x * y;
}

pub fn div(x: u32, y: u32) -> u32{
    return y/x;
}

fn main() {
    let x = 2;
    let y = 4;
    //mul(x,y);
    //div(x,y);
    println!("{} * {} = {}",x, y, mul(x,y));
    println!("{} / {} = {}",y, x, div(x,y));
}
