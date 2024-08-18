use std::io;

pub fn run(){
    // let mut input = String::new();
    // println!("Please input the first number");
    // io::stdin().read_line(&mut input).expect("Failed to read line");
    // println!("You typed:{}",input);  
    // let input1:f64 = match input.trim().parse(){
    //     Ok(num) => num,
    //     Err(_) => {
    //         println!("Please type a number");
    //         return;
    //     }
    // };

    // input.clear();

    // println!("Please input the second number");
    // io::stdin().read_line(&mut input).expect("Failed to read line");
    // println!("You typed:{}",input);
    // let input2:f64 = match input.trim().parse(){
    //     Ok(num) => num,
    //     Err(_) => {
    //         println!("Please type a number");
    //         return;
    //     }
    // };


    // // let result = division_option(input1, input2);
    // let result = division_result(input1, input2);   
    // match result{
    //     // Some(x) => println!("Result:{:.3}",x),
    //     // None => println!("Cannot divide by zero")
    //     Ok(x) => println!("Result:{:.3}",x),
    //     Err(e) => println!("{}",e)
    // }
    let a = [1,2];
    let result = sum(&a);
    match result{
        Some(x) => println!("Result:{}",x),
        None => println!("Out of index")
    }
}

// fn division_option(x: f64,y: f64) -> Option<f64>{
//     if y == 0.0{
//         None
//     } else {
//         Some(x/y)
//     }
// }

fn division_result(x: f64, y: f64) -> Result<f64, String>{
    if y == 0.0{
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(x/y)
    }
}

fn sum(a: &[i32]) -> Option<i32>{
    let a0 = a.get(0)?;
    let a1 = a.get(1)?;
    let a2 = a.get(2)?;
    Some(a0 + a1 + a2)
}