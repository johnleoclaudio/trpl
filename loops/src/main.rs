fn main() {
    let value = break_return_value();

    println!("The value is {value}");

    loop_labels();

    iterate();
    better_iterate();

    countdown();
}

fn break_return_value() -> i32 {
    let mut counter = 0;

    loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    }
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }
}

fn iterate() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut index = 0;

    while index < 6 {
        println!("The value is {}", a[index]);
        index += 1;
    }
}

fn better_iterate() {
    let a = [1, 2, 3, 4, 5, 6];

    for element in a {
        println!("The value is {element}");
    }
}

fn countdown() {
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}
