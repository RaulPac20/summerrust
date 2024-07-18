//Assignment 1
const FREEZING_POINT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * (9.0 / 5.0) + FREEZING_POINT
}

fn main() {
    let mut fahrenheit: f64 = 32.0; 

    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{:.2} Fahrenheit is {:.2} Celsius", fahrenheit, celsius);

    println!("Converting next 5 integer temperatures:");
    for i in 0..5 {
        let next_fahrenheit = fahrenheit + i as f64 + 1.0;
        let next_celsius = fahrenheit_to_celsius(next_fahrenheit);
        println!("{:.2} Fahrenheit -> {:.2} Celsius", next_fahrenheit, next_celsius);
    }
}


//Assignment 2
fn main() {
    let numbers = [15, 2, 30, 9, 10, 4, 7, 14, 6, 5];

    for &number in numbers.iter() {
        if is_even(number) {
            println!("{} is even", number);
        } else {
            println!("{} is odd", number);
        }

        if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz");
        } else if number % 3 == 0 {
            println!("Fizz");
        } else if number % 5 == 0 {
            println!("Buzz");
        }
    }
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);
    let mut largest = numbers[0];
    for &number in numbers.iter() {
        if number > largest {
            largest = number;
        }
    }
    println!("Largest number: {}", largest);
}
fn is_even(n: i32) -> bool {
    n % 2 == 0
}


//Assignment 3

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}
fn main() {
    let secret = 42; 
    let mut guess = 30; 
    let mut attempts = 0;

    loop {
        attempts += 1;

        match check_guess(guess, secret) {
            0 => {
                println!("Congratulations! You guessed the correct number: {}", guess);
                break;
            }
            1 => {
                println!("Your guess of {} is too high.", guess);
              
                guess -= 1;
            }
            -1 => {
                println!("Your guess of {} is too low.", guess);
                
                guess += 1;
            }
            _ => unreachable!(),
        }
    }
    println!("It took you {} attempts to guess the correct number.", attempts);
}