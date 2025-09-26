fn assignment1()
{
    const FREEZING_F: f64 = 32.0; //a constant for the freezing point of water in Fahrenheit
    
    fn fahrenheit_to_celsius(f: f64) -> f64 // function to convert from fahrenheit to celsius
    {
        (f - FREEZING_F) * 5.0 / 9.0
    }

    fn celsius_to_fahrenheit (c: f64) -> f64 // function to convert from celsius to fahrenheight
    {
        (c * 9.0 / 5.0) + FREEZING_F
    }

    let mut ftemp: f64 = 32.0; // mutable temperature in fahrenheit

    let ctemp = fahrenheit_to_celsius(ftemp);
    println!("{}°F = {}°C", ftemp, ctemp);

    let mut count = 0;
    while count < 5 // show the next 5 temperatures
    {
        ftemp = ftemp+1.0; // go up by one degree
        let ctemp = fahrenheit_to_celsius(ftemp);
        println!("{}°F = {}°C", ftemp,ctemp);
        count = count + 1;
    }

    let temp_c = 100.0;
    let temp_f = celsius_to_fahrenheit(temp_c);
    println!("Celsius to Fahrenheit Conversion: {}°C = {}°F", temp_c, temp_f);

}

fn assignment2()
{
        let numbers = [3,8,13,14,30,42,67,77,84,95]; // an array of 10 number

        fn is_even(n:i32) -> bool
        {
            n % 2 == 0 // function to check if number is even
        }

        for num in numbers
        {
            if num%3 == 0 && num%5 == 0
            {
                println!("{} --> FizzBuzz", num);
            }

            else if num%3 == 0 
            {
                println!("{} --> Fizz", num);
            }

            else if num%5 == 0
            {
                println!("{} --> Buzz", num);
            }

            else if is_even(num)
            {
                println!("{} --> Even", num);
            }

            else
            {
                println!("{} --> Odd", num);
            }
 
        }

        let mut sum = 0;
        let mut i = 0;
        while i < numbers.len() // totaling the sum
        {
            sum = sum + numbers[i];
            i = i+1;
        }

        println!("Sum = {}", sum);

        let mut largest = numbers[0]; // determine the largest number
        let mut j = 0;
        while j < numbers.len()
        {
            if numbers[j] > largest
            {
                 largest = numbers[j];
            }
            j = j + 1;
        }
        println!("Largest Number = {}", largest);
}

fn assignment3()
{
    let secret = 19;

    fn check_guess(guess: i32, secret: i32) -> i32
    {
        if guess == secret
        {
            0
        }
        else if guess > secret
        {
            1
        }
        else
        {
            -1
        }
    }

    let mut guess = 50;
    let mut tries = 0;

    loop
    {
        tries = tries+1;
        let result = check_guess(guess, secret);

        if result == 0
        {
            println!{"Guess {}: {} -> Correct!", tries, guess};
            break;
        }
        else if result == 1
        {
            println!{"Guess {}: {} -> Too high!", tries, guess};
            guess = guess -1;
        }
        else
        {
            println!{"Guess {}: {} -> Too low!", tries, guess};
            guess = guess +1;
        }
    }
    println!("It took {} guesses.", tries);

}

    



fn main() 
{
    println!("__Assignment 1__");
    assignment1();

    println!();

    println!("__Assignment 2__");
    assignment2();

    println!();
    
     println!("__Assignment 3__");
     assignment3();




   


}
