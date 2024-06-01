
pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a > b {
        println!("a is bigger than b");
        a
    } else {
        println!("b is bigger than a");
        b
    }
}

fn main() {
    let a =56;
    let b: i32 = 87;
    let result = bigger(a,b);

    // This is just a placeholder main function.
    println!("The bigger number between {} and {} is {}", a, b, result);
}