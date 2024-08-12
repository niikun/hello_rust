pub fn run(){
    // println!("This is ownership.rs");
    // let s1 = String::from("Hello");
    // let s2 = s1.clone();

    // println!("{},{}",s1,s2);

    // let i1 = 1;
    // let i2 = i1;

    // println!("Stack Address of i1 is {:p}: ",&i1);
    // println!("Stack Address of i2 is {:p}: ",&i2);

    // let sl1 = "hello";
    // let sl2 = sl1;
    // println!("Stack Address of sl1 is :{:p}",&sl1);
    // println!("Stack Address of sl2 is :{:p}",&sl2);

    // let sl3 = String::from("Hi");
    // println!("Stack Address of sl3 is :{:p}",&sl3);
    // println!("Heap Address of s3 is :{:p}",sl3.as_ptr());
    // println!("Heap length of s3 is :{}",sl3.len());
    // println!("Heap capacity of s3 is :{}",sl3.capacity());
    // take_ownership(sl3);
    // println!("{}",s3);
    let s6 = String::from("hello");
    println!("Stack Address of s6 is :{:p}",&s6);
    println!("Heap Adress of s6 in :{:p}",s6.as_ptr());
    println!("Heap length of s6 is :{}",s6.len());
    let mut s7 = take_giveback_ownership(s6);
    println!("Stack Address of s7 is :{:p}",&s7);
    println!("Heap Adress of s7 in :{:p}",s7.as_ptr());
    println!("Heap length of s7 is :{}",s7.len());

    let len = calc_len(&s7);
    println!("Length of {} is :{}",s7,len);

    let change_string = change(&mut s7);
    println!("{:?},{:?}",s7,change_string);
    println!("Stack Address of s7 is :{:p}",&s7);
    println!("Stack Address of change_string is :{:p}",&change_string);
    // println!("Heap Address of change_string is :{:p}",change_string.as_ptr());
    let mut s10 = String::from("hello");
    // let r2 = &s10;
    // let r1 = &mut s10;

    let mut s12 = String::from("Hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{},{}",r1,r2);
    let mut r3 = &mut s12;
    // r3 = String::from("Hello Japan");

    *r3 = String::from("Hello Japam");
    println!("{}",r3);
    // println!("{}",s10);
    // println!("{}",r1);
    println!("{}",s12);
    s12 = String::from("Hello World");
    println!("{}",s12);
}

// fn take_ownership(s: String){
//     println!("Stack Address of s is :{:p}",&s);
//     println!("Heap Adress of s in :{:p}",s.as_ptr());
//     println!("Heap length of s is :{}",s.len());
//     println!("Heap capacity of s is :{}",s.capacity());
//     println!("{}",s);
// }


fn take_giveback_ownership(s:String)->String {

    println!("Stack Address of s is :{:p}",&s);
    println!("Heap Adress of s in :{:p}",s.as_ptr());
    println!("Heap length of s is :{}",s.len());
    s    
}

fn calc_len(s: &String) ->usize{
    s.len()
}

fn change(s: &mut String){
    s.push_str(" world");
}