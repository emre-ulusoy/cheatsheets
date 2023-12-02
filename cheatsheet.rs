//* Variables
let lang = "Rust";
let mut lang = "Rust"; // mutable
let lang:char = 'R';
let lang:&str = "Rust"; //! Why '&'?
let lang; // Try not to declare w/o assigning
let (course,category) =("Rust","beginner");
const // it is mandatory to define the data type of const variables.

//* Print

//* Arrays
let arr:[i32;4] = [1, 2, 3, 4];
let arr:[i32;4] = [1, 2, 3, 4]; // mutable arrays can be edited
let arr1 = [0 ; 4]; // initialize an array of size 4 with 0
// Slice
let slice_array:&[i32] = &arr[0..2]; // Uses an existing array
let slice_array = &arr;

//* Functions
fn main() {}

//*Tuples
let data = ("Alex", 48, "70kg", "6ft");
let data : (&str, i32, &str, &str) = ("Alex", 48, "70kg", "6ft");
data.0 // -> "Alex", how we access touple values
let (w ,x, y, z) = data // Destructuring tuple

//* IF statements
//* Pretty much same as Go
// Ternary
let package_manager = if learn_language == "Rust" {"Cargo"} else {"Modules"};
//if let
if let (x, y) = coord {} else {}

//* Switch Case statement (Match)
match x {
    1 => println!("Python"),
    2 => println!("C++"),
    3 => {
        println!("Rust"); // Notice the ';' if using brackets {}
    },
    _ => println!("Some other value"),
};
// If you want to assign a value to the result variable from within the match block
let cource = "Rust";
let found_course = match course {
    "Rust" => "Rust",
    "C++" => "C Plus Plus",
    _ => "Unknown Language"
};

//* FOR Loops
for i in 0..5 {}
for (i, v) in (7..10).enumerate() {}

//* Other loops: while, loop.
// Loop 'labels
'outer:for i in 1..5 { //outer loop
   'inner:for j in 1..5 { // inner loop
        if i == 3 { continue 'outer; } // Continues the loop over `i`.
        if j == 2 { continue 'inner; } // Continues the loop over `j`.
   }
}

//* Functions
fn square(n:i32) -> i32 {}
fn square(mut n:i32) {} // Can change val inside the scope of the func without let
fn square(n:&mut i32) {} // Pass by reference, along with the param, the declaration of n and the
                         //func call argument needs to be mutable
fn square(n:i32) -> (i32, i32) {
    (a, b) // Same as `return (a, b);`
}
// Arrays as parameters & arguments
fn function_name( mut array_name:[i32;5]) {} // mut is so we can change the arr locally, but it isn't necessary
fn function_name(array_name:&mut [i32;12]) {} // pass by reference, same three part mut for the array
fn function_name()->[i32;10] {} // Return signature for an array