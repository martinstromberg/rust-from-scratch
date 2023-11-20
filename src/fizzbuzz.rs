pub fn fizz_buzz(n: i32) {
    if n <= 0 {
        return
    }

    fizz_buzz(n - 1);

    if n % 15 == 0 {
        println!("FizzBuzz")
    } else if n % 3 == 0 {
        println!("Fizz")
    } else if n % 5 == 0 {
        println!("Buzz")
    } else {
        println!("{}", n)
    }
}
