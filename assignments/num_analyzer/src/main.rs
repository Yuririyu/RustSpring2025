fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 15];
    //fizz loop
    for &num in &numbers {
        if is_even(num) {
            println!("{} is even", num);
        } else {
            println!("{} is odd", num);
        }

        if num % 3 == 0 && num % 5 == 0 {
            println!("{} is FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{} is Fizz", num);
        } else if num % 5 == 0 {
            println!("{} is Buzz", num);
        }
    }
    //sum loop
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);
    //largest num loop 
    let mut largest = numbers[0];
    for &num in &numbers {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}