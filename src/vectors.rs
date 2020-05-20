use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);
    numbers.pop();

    println!("{:?}", numbers);

    println!("Vector length: {}", numbers.len());

    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate value
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}
