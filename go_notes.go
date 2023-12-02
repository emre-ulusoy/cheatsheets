// * Golang 🦝

// * Packages/Libraries
package notes // Need this at the top of every Go file

import "lib"
// fmt: all the basic functions like print
// math:
// strings: funcs to manipulate strings

// * var declarations
var name string

var (
    age    int
    name, location  string // two at a time
    occupation string = "plumber"
    fav_color = "black" // inferred typing, omit type
    lastname, address = "Bob", "Chicago" // initialize mult vars
    weight := 190 // Inside a function, the := short assignment statement can be used in place of a
    // var declaration with implicit type.
    action := func() {
        // A variable can contain any type, including functions
    }
)
action() // executing the func from above

// * Common Functions & stuff
// strings.Fields(): split() in Python

// * Print
fmt.Printf("%s age %d", name, age) // %s: string, %d: decimal
// Prints the passed in variables’ values and appends a newline. fmt.Printf is used when you want to
// print one or multiple values using a defined format specifier (%d, %s).
// https://pkg.go.dev/fmt#hdr-Printing
print(name, " age ", age) // Built-in print func but more primitive (no space or newline)
// Verbs
// The format placeholders, such as %s or %d, are called verbs.
// Sprintf()??
// Use '+' to print the name and value instead of just val

// * constant vars
const Pi = 3.14
// Constants can only be a character, string, boolean, or numeric values and cannot be declared
// using the := syntax. An untyped constant takes the type needed by its context.

// * Main
// Programs start running in package main. Need a package main and func main() if it's an exe vs a
// lib. Nonstandard lib packages are namespaced using a web url.
package main
import "fmt"

func main() {
    fmt.Printf("Hello, World!\n")
}

// * Import/Export
// In Go, a name is exported if it begins with a capital letter.

// * Functions
// When declaring functions, the type comes after the variable name in the inputs.
// The return type(s) are then specified after the function name and inputs, before writing the
// definition. Functions can be defined to return any number of values and each of them are put
// here. Returned values can be named but try to avoid doing that.
func location(city, county string, zip int) (string, string) { /* */ }
// Two str variables, and two vals returned.

// * Pointers
// No pointer arithmetic. By default Go passes arguments by value. Pass pointers if you want to pass
// by reference.
// To get the pointer of a value, use the & symbol in front of the value; to dereference a pointer,
// use the * symbol.
// https://tour.golang.org/moretypes/1

// * Types
// The verb %T will print out the type. Variables need to be converted to another type explicitly.
var i int = 42
var f float64 = float64(i) // Like this
// More succinct:
i := 42
f := float64(i)

// * Struct
// Structs can be used to define additional types in Go. This is the reason for the keyword "type"
type Bootcamp struct {	// Example struct declaration.
    // Latitude of the event
    Lat float64
    // Longitude of the event
    Lon float64
    // Date of the event
    Date time.Time
}

func main() {
    fmt.Println(Bootcamp{
        Lat:  34.012836,
        Lon:  -118.495338,
        Date: time.Now(),
    })
}

type Point struct {
    X, Y int
}

var (	// Declaration of struct literals.
    p = Point{1, 2}  // has type Point
    q = &Point{1, 2} // has type *Point
    r = Point{X: 1}  // Y:0 is implicit
    s = Point{}      // X:0 and Y:0
)

// Struct tags: small pieces of metadata attached to fields of a struct that provide instructions to
// other Go code that works with the struct. Tag is offset with backtick ` characters.
type User struct {
    ID       int    `json:"id"`
    Name     string `json:"name"`
    Phone    string `json:"phone,omitempty"`
    Password string `json:"-"`
}
// A common use for struct tags is for encoding the data of your struct to some type of other
// format. JSON, XML, Protobuf, etc are examples of these types of encoding.

// New
// Go supports the new expression to allocate a zeroed value of the requested type and to return a
// _pointer_ to it.
x := new(int) // Pointer to zeroed int

x := new(Bootcamp)
y := &Bootcamp{}
fmt.Println(*x == *y) // The two are equivalent
fmt.Printf("%+v", p) // The value in a default format when -printing structs-, the plus flag (%+v)
                     // adds field names.

// * Map - Make
// Used to create maps (dict) when not using map literals
var m map[string]celebsAges
cities := map[string]int{}
cities := map[string]int{
        "New York":    8336697,
    }
delete(m, key) // Delete key in map m
elem, ok = m[key] // Checking if key "key" exists. Bool result in ok
for key, value := range cities {
    fmt.Printf("city: %s , no: %s\n", key, value)
}

// * Arrays
var a [10]int // declares a variable 'a' as an empty array of ten integers.
primes := [6]int{2, 3, 5, 7, 11, 13}
// An array’s length is part of its type, so arrays cannot be resized.
a := [...]string{"hello", "world"} // you can use an ellipsis to use an implicit length when you
                                   // pass the values.
a := [2][3]string // Multi-dimensional array.

// * Slices
// Slices don't have a length in the declaration!!!!
// Slices can be resized because they are a wrapper on top of arrays.
a := []int{2, 3, 5, 7, 11, 13} // Slice literal
s[lo:hi] // Slicing from lo to hi-1 inclusive
cities := make([]string, 3) // basically -> cities := [3]string
cities := []string{} // Empty slice, then append val(s)
cities = append(cities, "San Diego", "Ankara")
cities = append(cities, otherCities...) // Use ellipsis to append a slice to another
len(cities) // length of array/slice, -> cap(cities) -> similar

// * ... (ellipsis)
// Note that the ellipsis is a built-in feature of the language that means that the element is a
// collection. We can’t append an element of type slice of strings ([]string) to a slice of strings,
// only strings can be appended. However, using the ellipsis (...) after our slice, we indicate that
// we want to append each element of our slice.

// * Range
for i, v := range pow { /**/ } // Loops over the length of pow. i = idx, v = value. Can omit v, can
// also replace 'i' with '_' if you don't need either.
// Can use "break" or "continue".
for key, value := range cities { /**/ } // For when the iterable is a "map" (dict/hash).

// * IF statement
if answer < 0 { // No parenthesis, yes brackets.
    return "Wrong answer"
} else if answer == 42 {
    return "Right answer"
} else {
    return "Whatever"
}

if v := math.Pow(x, n); v < lim { // The if statement can start with a short statement to execute
    return v 					  // before the condition.
}

// * FOR loops (also WHILE loops)
for i := 0; i < 10; i++ { // No parenthesis, yes brackets.
    sum += i
}

for sum < 1000 { 	// We can leave the pre and post statements empty. Iterate as long as sum<1000.
    sum += sum		// Also is how we implement the while loop. Same as `for ; sum < 1000; {`
}

for { // Do something in a loop forever. Same as `for true`. Also how `do while` is implemented.
}

// * Switch Case statement
// 1. You can only compare values of the same type. 2. You can set an optional default statement to
// be executed if all the others fail. 3. You can use an expression in the case statement.
switch v {
    case 0:
        fmt.Println("even")
    case 3 - 2:				// Expression in a case statement.
        fmt.Println("odd")
    default:				// Default statement.
        fmt.Println("imaginary")
}
// You can also have multiple values in a case statement, separated by commas.
// The `fallthrough` statement at the end of a case lets you continue trying to match other cases.
// You can use a `break` statement inside your matched statement to exit the switch processing.
// Switch without a condition is the same as _switch true_.

// * Methods
// A method is a function that has a defined receiver, in OOP terms, a method is a function on an
// instance of an object.
// -Method receiver- comes between the _func_ keyword and the method name. Method receiver needs to
// be a _struct_ that's already (user) defined (actually it can be -any- defined type, not just
// struct). Any obj made from a struct that's a methods receiver can use that method through dot
// notation.
// Pointer Receivers
// There are two reasons to use a pointer receiver. First, to avoid copying the value on each method
// call (more efficient if the value type is a large struct). Second is so that the method can
// modify the value that its receiver points to.

// * File organization:
// imports -> consts -> vars -> types (as few structs per file as possible) -> funcs -> methods


// * defer
// A defer statement defers the execution of a function until the surrounding function returns.
// Deferred function calls are pushed onto a stack. When a function returns, its deferred calls are
// executed in last-in-first-out order.

// * Alias
// To define methods on a type you don’t “own”, you need to define an alias for the type you want to
// extend. Aliases can be used as method receivers.
type MyStr string //using MyStr as an alias for type string

// * Interface
// Defined by a set of methods. How we achieve polymorphism in Go.
type Intrfc interface { ... }
// Slice of an interface
myInterface := []Intrfc{}


// * Errors
// The fmt package’s various print routines automatically know to call the method when asked to
// print an error.








