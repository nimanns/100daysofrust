fn main() {
    let mut arr : [i32; 5] = [4,2,5,3,1];
    bubble_sort(&mut arr);
    println!("{:?}", arr);
}

fn bubble_sort(input: &mut [i32]) {
    let mut length : usize = input.len();
    while length!=0 {
        for i in 0..length - 1 {
            if input[i] > input[i+1]{
                let temp: i32 = input[i];
                input[i] = input[i+1];
                input[i+1]=temp;
            }
        }
        length -= 1;
    }
}
