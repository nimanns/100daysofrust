fn main() {
    let number : String = decimal_to_binary(245);
    print!("{}", number);
}

fn decimal_to_binary(mut input:i32) -> String {
    let mut out = String::from("");
    
    while input != 0 {
        out.push_str((input % 2).to_string().as_str());
        input = input / 2;
    }
    
    return out.chars().rev().collect()
}