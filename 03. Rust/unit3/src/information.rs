 /////// CONTROL FLOW //////////
// if statement
let _msg = if num == 5 {
    "five"
} else if num == 4 {
    "four"
} else {
    "other"
};

// Replace of ternarias
num = if a { b } else { c };
// Nested
num = if a {
    if x { y } else { y }
} else {
    c
};

// loop
loop{
    break;
}
// syntax 
// with one ' identify your loop with a tag 
// include the tag in break to indicate what cycle you want to end
'bob: loop {
    loop {
        loop {
            break 'bob;
        }
    }
}
// Same thing works with continue
'bob: loop {
    loop {
        loop {
            continue 'bob;
        }
    }
}
// Explanation
while dizzy() {
    // do stuff
}
// Same as while loop
loop {
    if !dizzy() { break }
    // do stuff
}
// Same as Do-While
// PD: In Rust do while doesnt exist
loop {
    // do stuff
    if !dizzy() { break }
}
// For 
for num of [7,8,9].iter() {
    // do stuff
}
for (x,y) of [(1,2),(3,4)].iter(){
    // do stuff
}
// Diff between these is when added = sign you add last number in loop
for num in 0..50 {
    //do stuff
}
for num in 0..=50 {
    //do stuff
}

/////////// String /////////
// Exist 6 types but 2 of them are more important
&str // Cannot be modify --> Internally made of a pointer and a len
String // Can be modified --> Internally made of a pointer, len and capacity; len != capacity
// You can say &str is a subset of a String
// Similar as JS I supposed String have methods
let hello = "Welcome, Coder"; // Default &str
let msg = "hello".to_string(); // String
let msg1 = String::from("hello"); 
// There's a whole complex issue by using index in Strings
// for that is better to usse nth()
msg.bytes(); // Get bytes
msg.nth(0); // Get first byte
msg.chars(); // Get chars
msg.nth(0); // Get first char
graphemes(msg, true); // Get graphemes
