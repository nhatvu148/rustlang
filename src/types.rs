/* Primitive types:
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Boolean: bool
Characters: char
Tuples
Arrays: fixed length
*/

pub fn run() {
    // Default is i32
    let x = 1;

    // Default is f64
    let y = 1.2;

    // Add explicit type
    let z: i64 = 123123123123123;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 < 1;

    let a1 = 'a';
    let face = '😂';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
