// Basic comments work the same as C

// This program does basic printing (with some complicated formatting)
fn main() {
    // ! after a method name is a macro.
    // Rust likes using macros
    println!("Hello World!");

    /*
     * We do multi-line comments the same as C too.
     */

    // Variables require "let"
    let x = 5 + /* 90 + */ 5;
    // println can replace curly braces with variables generically
    println!("'x' should be 10, not 100... x = {}", x);

    // This can be combined with indices to specify which parameter you want
    println!("{0} is the first parameter, {2} is the third, {1} is the second", "Test", "'Second'", "'This'");

    // The following line won't compile because there are unused parameters!
    // println!("{0} is the first parameter, {2} is the third", "Test", "Unused", "'This'");

    // You can also specify parameters by name
    println!("My dog is called {dog}, my cat is called {cat}", dog="Ruby", cat="Trashbag");

    // You can pad text with whitespace or specifc values to reach a specific width
    println!("{animal:=>10}", animal="Panda"); // =====Panda

    // Only types that implement fmt::Display can be formatted with {}, which
    // I'll need to bear in mind for AOC! Structures don't do this by default.

    // You can also use variables instead of parameters, the macro will figure
    // it out!
    // Also, look! There are types here
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}"); // $ here is telling us that it is a usize type

    // You can control the number of decimals shown by using . to specify precision
    let pi = 3.141592;
    println!("{pi:.3}");
}
