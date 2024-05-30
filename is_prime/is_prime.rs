fn main() {
    println!("{}",is_prime(229));
}

fn is_prime(num:i32)->i32{
    if num <= 1 {
        return 0
    } else {
        for i in 2..((f32::powf(num as f32,0.5)) as i32 + 1){
            if num % i == 0 {
                return 0
            }
        } 
    }
    
    return 1
}