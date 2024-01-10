fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
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
    println!("End count = {}", count);
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
