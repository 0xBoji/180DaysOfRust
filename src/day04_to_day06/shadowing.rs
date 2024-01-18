pub fn run(){
    //Init a variable
    let x = 5;
    let x = x * 2;
    println!("x = {}", x);
    /*
    In this example, let x = 5; initializes x with the value 5. Then, let x = x * 2; creates a new variable also named x, which "shadows" the original x. This isn't modifying the original x (that would be mutability), but rather creating a completely new variable with the same name. The new x has a value of 10 (5 * 2). 
    */

    /*Key point:
    Shadowing is a useful tool in Rust, allowing you to reuse variable names without losing the immutability of the original variables and offering flexible transformation in terms of data types.
     */
}
