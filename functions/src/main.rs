fn main() {
    println!("Hello, world!");
    another_function();
    let name = "baldur";
    greet(name);

    let area = get_area(2.0, 2.0);
    println!("The area is: {area}");

    // Statements and Expressions
    //
    // Statements are instructions that do some
    // actions but don't return a value
    //
    let i = 1;
    let y = { 5 + five() };

    println!("The sum is: {}", i + y);
}

fn another_function() {
    println!("Another function!")
}

fn greet(name: &str) {
    println!("Hello, {name}")
}

fn get_area(height: f64, width: f64) -> f64 {
    height * width
}

fn five() -> i32 {
    5
}
