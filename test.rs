fn main(){
    let x = do_stuff(2.0,3.5);
    println!("ans = {}",x);
}


fn do_stuff(a:f64,b:f64)->f64{
    println!("{}*{}",a,b);
    a*b
}