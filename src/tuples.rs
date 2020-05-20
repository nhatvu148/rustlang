pub fn run() {
    let person: (&str, &str, i8) = ("Vu", "cool", 18);

    println!(
        "{} is {} and {} is {}",
        person.0, person.1, person.0, person.2
    );
}
