use rand::Rng;

fn main() {
    let mut arr: Vec<i32> = Vec::with_capacity(10);
    
    for _ in 0..arr.capacity() {
        arr.push(rand::rngs::OsRng.gen_range(0..=10));
    };
    selection_sort(&mut arr);
    println!("{:?}", arr);
}

fn selection_sort(input: &mut [i32]) {
    for i in 0..input.len() {
        for j in i+1..input.len() {
            if input[j] < input[i] {
                let temp = input[j];
                input[j] = input[i];
                input[i] = temp
            }
        }
    }
}
