use unit2::greet;
use rand::prelude::*;

fn main() {
    // Aqui lo llamamos
    // let x = do_stuff(2.0,12.5);
    greet();
    // starts working when made it public
    // If horrible to use all route
    println!("Que cosa tocara: {}", rand::rng().random_range(1..=100));
}

// Aqui veremos funciones
fn _do_stuff(qty: f64, oz: f64) -> f64 {
    // Si una linea no tiene ; se interpreta que alli esta el return
    // esta linea es igual a return qty* oz;
    println!("{} {}-oz sarsaparrilla(s)!", qty, oz);
    qty * oz
}


// // Silence some warnings so they don't distract from the exercise.
// #![allow(unused_variables)]

// fn main() {
//     let width = 4;
//     let height = 7;
//     let depth = 10;
//     // 1. Try running this code with `cargo run` and take a look at the error.
//     //
//     // See if you can fix the error. It is right around here, somewhere.  If you succeed, then
//     // doing `cargo run` should succeed and print something out.
    
//     println!("Area is {}", area_of(width, height));

//     // 2. The area that was calculated is not correct! Go fix the area_of() function below, then run
//     //    the code again and make sure it worked (you should get an area of 28).

//     // 3. Uncomment the line below.  It doesn't work yet because the `volume` function doesn't exist.
//     //    Create the `volume` function!  It should:
//     //    - Take three arguments of type i32
//     //    - Multiply the three arguments together
//     //    - Return the result (which should be 280 when you run the program).
//     //
//     // If you get stuck, remember that this is *very* similar to what `area_of` does.
//     //
//     println!("Volume is {}", volume(width, height, depth));
// }

// fn area_of(x: i32, y: i32) -> i32 {
//     // 2a. Fix this function to correctly compute the area of a rectangle given
//     // dimensions x and y by multiplying x and y and returning the result.
//     //
//     x * y
//     // Challenge: It isn't idiomatic (the normal way a Rust programmer would do things) to use
//     //            `return` on the last line of a function. Change the last line to be a
//     //            "tail expression" that returns a value without using `return`.
//     //            Hint: `cargo clippy` will warn you about this exact thing.
// }

// fn volume(x: i32, y: i32, z: i32) -> i32 {
//     x * y * z
// }