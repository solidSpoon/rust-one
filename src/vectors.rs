// Vectors - Resizable Vectors

use std::mem;

pub fn run() {
    let mut  numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    //Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();


    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    //Get Vector length
    println!("Vector Length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vectors
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}