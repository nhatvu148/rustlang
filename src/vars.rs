pub fn run() {
    let name = "Nhat Vu";
    let mut age = 28;
    println!("My name is {} and I am {}", name, age);

    age = 30;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Vu", 18);
    println!("{} is {}", my_name, my_age);
}
