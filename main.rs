use rand::Rng;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    let mut sorted = false;
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                sorted = false;
            }
        }
        n -= 1;
    }
}

fn main() {

    

    let mut map = HashMap::new();
    let mut rng = rand::thread_rng();

    for i in 1..=10 {
        let mut my_vector = Vec::new();
        for _ in 0..10 {
            let random_number = rng.gen_range(1..=1000);
            my_vector.push(random_number);
        }

        println!("\n\t\tBEFORE (vector {})", i);

        thread::sleep(Duration::from_secs(2));

        for num in &my_vector {
            println!("{num}", num = num);
        }

        thread::sleep(Duration::from_secs(3));

        bubble_sort(&mut my_vector);

        println!("\n\t\tAFTER (vector {})", i);

        thread::sleep(Duration::from_secs(2));

        for num in &my_vector {
            println!("{num}", num = num);
        }

        thread::sleep(Duration::from_secs(3));

        map.insert(i, my_vector);
    }

    println!("\n\nAccessing sorted vectors in the hashmap:");
    for (i, sorted_vector) in map.iter() {
        println!("vector {}: {:?}", i, sorted_vector);
    }
}



