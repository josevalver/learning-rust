// 1. A variable can be used only if it has been initialized
/*
fn main() {
    let x: i32; // Unititialized and used, ERROR !
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
*/
fn main() {
    let x: i32 = 5; // Now it's initialized, so "Success!" is printed
    let y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}