fn main() {
    let mut arr : [i32; 5] = [4,2,5,3,1];
    insertion_sort(&mut arr);
    println!("{:?}", arr);
}

fn insertion_sort(input: &mut [i32]) {
    let length : usize = input.len();
    for i in 0..length {
        let mut j : usize = i;
        while j > 0 && input[j-1] > input[j] {
            let temp = input[j];
            input[j] = input[j-1];
            input[j-1] = temp;
            j = j - 1;
        }
    }
}
