use rand::Rng;

fn main() {
    let mut arr: Vec<i32> = Vec::with_capacity(10);
    
    for _ in 0..arr.capacity() {
        arr.push(rand::rngs::OsRng.gen_range(0..=10));
    };
    
    // let mut arr : Vec<i32> = [7, 0, 6, 10, 5, 4, 10, 7, 10, 1].to_vec();
    println!("{:?}", arr);
    heap_sort(&mut arr);
    println!("{:?}", arr);
}

fn heap_sort(mut input: &mut [i32]) {
    for i in (0..input.len()/2-1).rev() {
        heapify(&mut input, i, 10);
    }
    
    for i in (1..input.len()).rev() {
        let temp = input[0];
        input[0] = input[i];
        input[i] = temp;
        
        heapify(&mut input, 0, i)
    }
}

fn heapify(input: &mut [i32], curr: usize, size: usize) {
    let mut largest = curr;
    let left = 2 * curr + 1;
    let right = 2 * curr + 2;
    
    if left < size && input[left] > input[largest] {
        largest = left;
    }
    
    if right < size && input[right] > input[largest] {
        largest = right;
    }

    if largest != curr {
        let temp = input[curr];
        input[curr] = input[largest];
        input[largest] = temp;
        heapify(input, largest, size);
    }

}