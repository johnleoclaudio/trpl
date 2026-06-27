fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value is: {number}");

    let x;
    if condition {
        x = 3;
    } else {
        x = 4;
    }

    println!("The x value is: {x}")
}
