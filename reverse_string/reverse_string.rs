fn main() {
    println!("{}",reverse_string("test"));
}

fn reverse_string(input:&str)->String{
    let mut output = String::from("");
    for x in (0..input.len()).rev() {
        output.push(input.as_bytes()[x] as char);
    }
    return output
}