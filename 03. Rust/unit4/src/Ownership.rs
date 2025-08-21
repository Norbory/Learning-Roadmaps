///////// OWNERSHIP
// In Rust, we have 3 rules
// 1. Each value in Rust has a variable that’s its “owner”.
// 2. A value can only have one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// If you just borrow the value of S1 to S2
// What is really happening?
let s1 = String::from("abc");
// You are moving the ownership of the value from s1 to s2
// For that reason, when you try to use s1 after this line, it will result in an error
// S1 still exists, but it's no longer accessible
// and due to you didn't type it as mutable
// S1 stack can't be modified
//let s2 = s1;
// Instead, you must clone it
// Because, the program create a copy of the whole heap
// S1 is still valid
let _s2 = s1.clone();
print!("{}", s1);

let mut s3 = String::from("Onta bb");
// What is really happening?


//////// BORROWING & REFERENCES
// the & before the variable represents a reference
// s create a pointer to S1 stack and heap
// do_stuff(&s3);
// we also can send a mutable reference
println!("{}", s3);
do_things(&mut s3);
println!("{}", s3); // S3 is still valid because we just borrowed it

// fn do_stuff(s: &String) {
//     println!("{}", s);
// }
fn do_things(s: &mut String) {
    s.push_str("ghi");
    // We can also dereference the reference
    // This allows us to modify the original value
    *s = String::from("Aqui ta");
}

// It is no recomended to have lots of mutable references at the same time
// But you can have multiple immutable references