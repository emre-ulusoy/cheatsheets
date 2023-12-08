//* Variables
let lang = "Rust";
let float32 = 5_f32; // initializing with the right type of number
let mut lang = "Rust"; // mutable
let lang:char = 'R';
let lang:&str = "Rust"; //! Why '&'?
let lang; // Try not to declare w/o assigning
let (course,category) =("Rust","beginner");
const // it is mandatory to define the data type of const variables.
//* Slices (string)
let slice = &lang[1..3]; // Slice of one of the strings from the above examples

//* Print

//* Arrays
let arr:[i32;4] = [1, 2, 3, 4];
let arr:[i32;4] = [5, 6, 7, 8]; // mutable arrays can be edited
let arr1 = [0 ; 4]; // initialize an array of size 4 with 0
//* Slice (array)
let slice_array:&[i32] = &arr[0..2]; // Uses an existing array
let slice_array = &arr;
//* Vectors
let my_vec = vec![1, 2, 3, 4, 5];
let my_vec:Vec<optional_type_size> = vec![1, 2, 3, 4, 5]; // Extra optional stuff
// Iterating over a vector, or just use `.iter()` in a loop
let index = my_vec.iter().position(|&r| r == lang).unwrap();
//* Slice (vector)
let my_slice:&[i32] = &my_vec[2..4];

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
for found in str.chars() {}
//* Other loops: while, loop.
// Loop 'labels
'outer:for i in 1..5 { //outer loop
   'inner:for j in 1..5 { // inner loop
        if i == 3 { continue 'outer; } // Continues the loop over `i`.
        if j == 2 { continue 'inner; } // Continues the loop over `j`.
   }
}

//* Functions
fn square(n:i32) -> i32 {} // return types are indicated by an arrow, unlike the nothing in Go
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

//* Struct
struct StructName{ // Has to be PascalCase
    itemName:datatype,
}
let instance_name = StructName { // Initializing a struct
    itemName: value,
}
//* Methods (in structs)
impl StructName { // Assuming the struct is already defined
    fn method_name(&self) -> String {
        format!("{} {}", self.name, self.code)
    }
}
// Tuple structs
struct TupleStruct(i32, String, &str)
let ts1 = TupleStruct(5, "emre".to_string(), "emre")

//* Enums
enum EnumName{ // Defining. PascalCase
    Variant1,
    Variant2,
}
let inst_name = EnumName::Variant; // Initializing
// Glob Operator ( * )
use EnumName::*;
let inst_name = Variant;
//* Methods (in enums)
impl EnumName {
    fn method_name(&self) -> bool {
        match self{
            EnumName::Variant1=> return true,
            _=> return false,
        }
    }
}

//* Traits
trait TraitName { // Like interfaces in Go
    fn trait_method(&self)->f32;
 }
 impl TraitName for StructName { // Can do this for multiple structs
    fn trait_method(&self)->f32 {
        self.height * self.width
    }
 }

//* Modules
mod outer_module { // outer_module::inner_module::my_public_function();
    fn private_function() {
      println!("Hi, I got into the private function of outer module");
    }
    pub mod inner_module { // Public module
      pub fn public_function() {
        super::private_function();
      }
    }
}
// Importing mods/files from the same dir
mod file_name // Import
file_name::module_name::x // Calling the module function/array/trait/struct
file_name::x // If no module name, file name implicitly becomes module name

//* Lifetimes
fn fun_name<'a , 'b>(x: & 'a i32 , y: & 'b i32)