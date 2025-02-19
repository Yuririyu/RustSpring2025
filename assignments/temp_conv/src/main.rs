const FREEZING: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING
}

fn main() {
    let mut temp_f = 32.0; // Starting temperature in Fahrenheit
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{}°F is {:.2}°C", temp_f, temp_c);

    for i in 1..=5 {
        let next_f = temp_f + i as f64;
        let next_c = fahrenheit_to_celsius(next_f);
        println!("{}°F is {:.2}°C", next_f, next_c);
    }
}