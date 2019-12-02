// Day1 solution code
use std::io;

fn get_fuel(mass : i32) -> i32 {
    if mass < 9 { return 0; }
    return mass / 3 - 2;
}

fn main() {
    let mut total_fuel: i32 = 0;

    // Get fuel for module weights
    let mut input = String::new();
    loop {
        input.clear();
        let mut fuel_for_module: i32 = 0;
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 { // Reached EOF
                    break;
                }
                match input.trim().parse::<i32>() {
                    Ok(mass) => {
                        fuel_for_module = get_fuel(mass);
                    }
                    Err(error) => {
                        println!("parse failed on input: {}\nerror: {}", input, error)
                    }
                }
            }
            Err(error) => println!("error: {}", error),
        }

        // Get fuel for fuel
        let mut total_module_fuel = fuel_for_module;
        let mut new_fuel_mass: i32 = get_fuel(fuel_for_module);
        while new_fuel_mass > 0 {
            total_module_fuel += new_fuel_mass;
            new_fuel_mass = get_fuel(new_fuel_mass);
        }
        total_fuel += total_module_fuel;
    }

    println!("Total fuel: {}\n", total_fuel);
}
