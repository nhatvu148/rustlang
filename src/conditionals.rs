pub fn run() {
    let age: u8 = 18;
    let check_id: bool = false;

    if age >= 21 && check_id {
        println!("Age is larger than 21");
    } else if age < 21 && check_id {
        println!("Sorry");
    } else {
        println!("Ha ha");
    }

    // Shorhand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
