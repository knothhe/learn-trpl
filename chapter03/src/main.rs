fn main() {
    // variables_mut();
    // variables_shadowing();
    // tuple_type();
    // array_type();
    // repeat_with_loop();

    // convert temperature
    let fahrenheit = convert_temperature(30.0);
    println!("The value of fahrenheit is: {}", fahrenheit);

    // fibonacci
    let fib = fibonacci(10);
    println!("The value of fib is: {}", fib);
}

fn variables_mut() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

fn variables_shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}

fn tuple_type() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);
    let mut tup: (i32, i32) = (1, 2);
    tup.0 = 0;
    tup.1 += 5;
    println!("The value of tup.0 is: {}", tup.0);
    println!("The value of tup.1 is: {}", tup.1);
}

fn array_type() {
    let arr = [1, 2, 3, 4, 5];
    println!("The value of arr[0] is: {}", arr[0]);
    println!("The value of arr[1] is: {}", arr[1]);
    println!("The value of arr[2] is: {}", arr[2]);
    println!("The value of arr[3] is: {}", arr[3]);
    println!("The value of arr[4] is: {}", arr[4]);
}

fn repeat_with_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn convert_temperature(celsius: f64) -> f64 {
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    return fahrenheit;
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
