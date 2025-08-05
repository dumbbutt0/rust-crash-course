#![allow(unused)]
// Exercise: Fix the code to make it compile and pass the assertions

fn main() {
    let test =4;
    let  mut age = 17;
    let  name = "jake";
    // Exercise 1: Make this variable mutable
    let mut count = 1;
    println!("count: {count}");
    count += 1;
    println!("count: {count}");
    println!("my name is {0} and im {1} now, in {2} years ill be {3}",name,age,test,(age+test));    
}
