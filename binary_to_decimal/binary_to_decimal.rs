fn main() {
    print!("{}", binary_to_decimal("0010101101"));
}

fn binary_to_decimal(input: &str) -> i32 {
    let mut ch = input.chars().rev();
    let mut output : i32 = 0;
    for i in 0..input.len() {
        match ch.next(){
            Some(x) => output += x.to_digit(10).unwrap() as i32 * 2_i32.pow(i as u32),
            None => (),
        }
    }
    
    return output
}