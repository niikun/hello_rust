fn main(){
    // x is mutable variable
    let x = 5;
    println!("The values of x is {x}");
    let x = x + 1;
    {
        let x = x * 2;
        println!("The values of x is {x} in scope.");
    }
    println!("The values of x is {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The values of spaces is {spaces}");

    const Y:i32 = 10;
    println!("The Constant values of Y is {Y}");
    // const Y = 20;
    // println!("The Constant values of Y changes {Y}");
}
