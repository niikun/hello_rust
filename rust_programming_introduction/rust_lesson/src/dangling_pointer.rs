pub fn run(){
    let r;
    {
        let x = 5;
        r = &x;
        println!("{}",r);
    }
    // println!("{}",r);
}