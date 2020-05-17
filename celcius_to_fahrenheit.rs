use std::io::stdin;


fn celcius_to_fahrenheit(c_number: f32) -> f32 {
    (c_number * 9.0 / 5.0) + 32.0
}

fn main() {
    println!("Celcius to Fahrenheit converter!\n Type ':q' to exit");

    loop {
        println!("Enter a number:");
        let mut number_to_convert = String::new();

        stdin()
            .read_line(&mut number_to_convert)
            .expect("Failed to read line");

        let number_to_convert: f32 = match number_to_convert.trim() {
            ":q" => {
                println!("Goodbye!");
                break; },
            _ => match number_to_convert.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            }
        };

        println!("Result is {:#?} fahrenheit", celcius_to_fahrenheit(number_to_convert));
    }
}
