fn main() {
    println!("{}",factorial(10));
}

fn factorial(num:i32)->i32{
    if num == 0 {
        return 1
    } else {
        return num * factorial(num-1)        
    }
}