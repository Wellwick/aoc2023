
// Functions can have (typed parameters) and a return type
fn add_one(value : i32) -> i32
{
    // return values are specified by a semicolonless statement
    value + 1
}

fn selector(should_we : bool) -> i32
{
    // if conditionals produce blocks, which themselves can produce values
    // They're all just their own methods, if you want them to be. Cascade up
    // your values when necessary
    if should_we
    {
        100
    }
    else
    {
        0
    }
}

fn main()
{
    let x : i32 = 10;
    println!("{0} + 1 = {1}", x, add_one(x));

    // variables are scoped and inner scopes can override outer, then be
    // forgotten about again
    {
        let x = "square";
        println!("My favourite shape is {}", x); // What a loser...
    }

    // Value restored
    println!("All hail the return of the true x: {}", x);

    // Blocks can return values, so even scoped we can useful things out
    let x = 
    {
        let y = x; // This is the x from outside of scope
        let x = 20; // This is a newly scoped x
        let z = 12;
        x + y + z
        // You can still do
        // return x + y + z;
        // if you like, but do you need to?
    };

    println!("Despite scoping shenanigans, x is now {}", x);

    println!("Likelihood of occurring is {}%", selector(true));

}