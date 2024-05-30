use rand::Rng;

fn main() {
    let mut arr: Vec<i32> = Vec::with_capacity(10);
    
    for _ in 0..arr.capacity() {
        // arr.push(rand::random::<i32>());
        arr.push(rand::rngs::OsRng.gen_range(0..=10));
    };
    
    println!("{:?}", arr);
    quick_sort(&mut arr, 0, 9);
    println!("{:?}", arr);
}

fn quick_sort(input: &mut [i32], low_index : usize, high_index: usize) {
    
    if low_index >= high_index {
        return
    }
    
    let pivot : i32 = input[high_index];
    
    let mut left_pointer = low_index;
    let mut right_pointer = high_index;
    
    while left_pointer < right_pointer {
        while input[left_pointer] <= pivot && left_pointer < right_pointer {
            left_pointer += 1;
        }
        
        while input[right_pointer] >= pivot && left_pointer < right_pointer {
            right_pointer -= 1;
        }
        
        let temp = input[left_pointer];
        input[left_pointer] = input[right_pointer];
        input[right_pointer] = temp;
    }
    

    let temp = input[left_pointer];
    input[left_pointer] = input[high_index];
    input[high_index] = temp;
    
    if left_pointer != 0 {
        quick_sort(input, low_index, left_pointer-1);
        quick_sort(input, left_pointer+1, high_index);
    } else {
        quick_sort(input, low_index, left_pointer);
        quick_sort(input, left_pointer+1, high_index);
    }
}
