use std::{ io };

fn main() {
    println!("Convert to Celsius or Fahrenheit?");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");

    loop {
        println!("Please input your type");

        let mut temp_type = String::new();

        io::stdin()
            .read_line(&mut temp_type)
            .expect("Failed to read input");

        let temp_type = match temp_type.trim().parse() {
            Ok(1) => 1,
            Ok(2) => 2,
            _ => {
                println!("Please input 1 or 2!");
                continue;
            }
        };

        println!("Please input the temperature value: ");
        let mut temp_value = String::new();
        io::stdin().read_line(&mut temp_value)
            .expect("Failed to read line");

        let temp_value = match temp_value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input valid temperature value");
                continue;
            }
        };

        match temp_type {
            1 => {
                println!("{} Fahrenheit is {} Celsius", temp_value, ftoc(temp_value));
                break;
            },
            2 => {
                println!("{} Celsius is {} Fahrenheit", temp_value, ctof(temp_value));
                break;
            }
            _ => {
                println!("Error");
                break;
            }
        };
    }
}

// Celsius to Fahrenheit
fn ctof(c: i32) -> i32 {
    (c * (9 / 5)) + 32

}

//Fahrenheit to Celsius
fn ftoc(f: i32) -> i32 {
    (f-32) * (5 / 9)
}