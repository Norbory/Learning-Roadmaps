use unit3::*;

fn main() {
    #![allow(dead_code, unused_mut, unused_variables)]
    // In Rust, you can put undescores (_) in your values and they won't be read them
    let _x: u32 = 4_0_0_0;
    // These can be used to give your variables the type they are
    let _y = 4000_u32;
    // x and y have equal values and same types
    let _number = 500.78f32;
    let _number1 = 500.78_f32;
    // the same occurs for these other two
    // Use undescores to help readability of your code

    // Info structures
    /////////////////////////// Tuple
    // Help you manage different types values in same list
    let _info = (1,3.3,999); // they can be without types
    let _infor: (u8,f64,i32) = (1,3.3,999); // and aslo like this
    // methods
    // retrieve one each
    let _jets = _info.0;
    let _fuel = _info.1;
    let _ammo = _info.2;
    // All at once
    let (_jets1,_fuel1,_ammo1) = _info;
    // Pd: Tuple have an heredy of 12 items, after that methods will be pretty unefficient
    ////////////////////////// Array
    // multiple items but with the same type
    //let _buf = [1,2,3];
    //let _buf = [0;3]; // Both are ways to create one, except that for the second one you first put the value and then how many times you want that item.
    let _buf: [u8;3] = [1,2,3]; // To type, use this structure all the time with the type and how many items you have
    _buf[0];
    // As equal from the previous case, array have a bound of 32 items before reducing its effectiveness

    // Exercises 3
    //1.
    let coords: (f32, f32) = (6.3, 15.0);
    showdifference(coords.0, coords.1);
    //2.
    let coordsarray : [f32;2] = [6.3,15.0];
    print_array(coordsarray);
    //3.
    let series = [1, 1, 2, 3, 5, 8, 13];
    ding(series);
    //4.
    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0);
    //5.
    // Put all functions in libraries
    // ---------------------------------------------------------------------------//
    // Exercises D
    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    // This consumes the `args` vector to iterate through each String
    for arg in args {
        // 1a. Your task: handle the command-line arguments!
        //
        // - If arg is "sum", then call the sum() function
        // - If arg is "double", then call the double() function
        // - If arg is anything else, then call the count() function, passing "arg" to it.
        if arg == "sum" {
            sum();
        } else if arg == "double" {
            double();
        } else {
            count(arg);
        };

        // 1b. Now try passing "sum", "double" and "bananas" to the program by adding your argument
        // after "cargo run".  For example "cargo run sum"
    }
    

    fn sum() {
        let mut sum = 0;
        // 2. Use a "for loop" to iterate through integers from 7 to 23 *inclusive* using a range
        // and add them all together (increment the `sum` variable).  Hint: You should get 255
        // Run it with `cargo run sum`
        for num in 7..=23 {
            sum += num;
        };

        println!("The sum is {}", sum);
    }

    fn double() {
        let mut count = 0;
        let mut x = 1;
        // 3. Use a "while loop" to count how many times you can double the value of `x` (multiply `x`
        // by 2) until `x` is larger than 500.  Increment `count` each time through the loop. Run it
        // with `cargo run double`  Hint: The answer is 9 times.
        while x < 500 {
            x *= 2;
            count += 1;
        };

        println!("You can double x {} times until x is larger than 500", count);
    }

    fn count(arg: String) {
        // Challenge: Use an unconditional loop (`loop`) to print `arg` 8 times, and then break.
        // You will need to count your loops, somehow.  Run it with `cargo run bananas`
        //
        // print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.
        const MAX_COUNT: u32 = 8;
        let mut count = 0;
        loop {
            print!("{} ", arg);
            count += 1;
            if count == MAX_COUNT { break };
        }
        println!(); // This will output just a newline at the end for cleanliness.
    }
}