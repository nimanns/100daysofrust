fn main() {
    let mut arr: Vec<i32> = Vec::with_capacity(5);
    
    for _ in 0..arr.capacity() {
        arr.push(rand::random::<i32>());
    };
    
    merge_sort(&mut arr);
    println!("{:?}", arr);
}

fn merge_sort(input: &mut [i32]) {
    let input_length : usize = input.len();
    
    if input_length < 2 {
        return
    }
    
    let mid_index : usize = input_length / 2;
    let mut left_half : Vec<i32> = vec![Default::default(); mid_index];
    let mut right_half : Vec<i32> = vec![Default::default(); input_length - mid_index];

    
    for i in 0..mid_index {
        left_half[i] = input[i];
    }
    
    for i in mid_index..input_length {
        right_half[i - mid_index] = input[i];
    }
    
    merge_sort(&mut left_half);
    merge_sort(&mut right_half);
    
    // merge
    
    let left_length : usize = left_half.len();
    let right_length : usize = right_half.len();
    
    let (mut i, mut j, mut k) = (0, 0, 0);
    
    while i < left_length && j < right_length{
        if left_half[i] <= right_half[j] {
            input[k] = left_half[i];
            i += 1;
        } else {
            input[k] = right_half[j];
            j += 1;
        }
        k += 1;
    }
    
    while i < left_length {
        input[k] = left_half[i];
        i += 1;
        k += 1;
    }
    
    while j < right_length {
        input[k] = right_half[j];
        j += 1;
        k += 1;
    }
}
