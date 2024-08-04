// fn main() {
//     another_function(5);
// }

// fn another_function(x:i32){
//     println!("The value of x is: {}",x);
// }

// fn main(){
//     another_function(5,"h");
// }

// fn another_function(x:i32,y:&str){
//     println!("The value is {}{} .",x,y)
// }

// fn five() ->i32{
//     5
// }

// fn main(){
//     let x = five();
//     println!("value is {}",x);
// }


fn main(){
    let x = plus_one(5);
    println!("value is {}",x);
}

fn plus_one(x:i32) ->i32{
    x + 1
}