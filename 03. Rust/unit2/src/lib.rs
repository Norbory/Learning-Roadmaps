// By default this is private so nor a field in the same project could use it
// Same as C# with namespaces add them this way
use unit2::greet;

pub fn greet() {
    println!("Hi!");
}