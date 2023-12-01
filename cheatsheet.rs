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