// mod sub_a;
// mod sub_b;

const MAX_POINTS:u32 = 100_000;

pub fn run(){
    println!("Here is vars module");

    let mut x = 5;
    println!("Value of x is: {}", x);
    x = 6;
    println!("Value of x is: {}", x);    
    let _i1 = 3;
    let _f1 = 3.14;
    println!("MAX_POINTS is: {}", MAX_POINTS);
    println!("{}",usize::BITS);
    println!("memory address of const is :{:p}",&MAX_POINTS);
    let i2 :i64 = 3;
    let i3 :i64 = 2;
    println!("memory address of i2 is :{:p}",&i2);
    println!("memory address of i3 is :{:p}",&i3);

    let t1 = (500,6.4,"dummy");
    println!("t1 is: {:?}",t1);
    let ( _x, _y, _z) = t1;
    println!("{},{},{}",t1.0,t1.1,t1.2);
    let mut t2 = ((0,1),(2,3));
    let ((ref mut x1_ptr, ref mut y1_ptr),_) = t2;
    println!("{} {}" ,x1_ptr,y1_ptr);
    println!("{:p} {:p}" ,&x1_ptr,&y1_ptr);
    *x1_ptr = 5;
    *y1_ptr = 6;
    println!("{} {}" ,x1_ptr,y1_ptr);
    println!("{:p} {:p}" ,&x1_ptr,&y1_ptr);

    let a1 = [1,2,3,4,5];
    let a2 = [0;10];
    println!("a1 is :{:?}",a1);
    println!("a2 is :{:?}",a2);
    println!("a1.0 is :{}",a1[0]);
    println!("a2.1 is :{}",a2[1]);
    let s1 = "helloこんにちは挨拶";
    let s2 = "hello";

    println!("Stack Address of s1 is :{:p}",&s1);
    println!("Stack Address of s2 is :{:p}",&s2);
    println!("Static Address of s1 is :{:?}",&s1.as_ptr());
    println!("Static Address of s2 is :{:?}",&s2.as_ptr());
    println!("Static Address of s1 is :{}",&s1.len());
    println!("Static Address of s2 is :{}",&s2.len());

    let mut s1 = String::from("hello world");
    let mut s2 = String::from("hello");
    println!("Stack Adress of s1 is : {:p}",&s1);
    println!("Heap Address of s1 is : {:p}",s1.as_ptr());
    println!("Heap length of s1 is : {}",s1.len());
    println!("Heap capacity of s1 is : {}",s1.capacity());
    println!("====================");

    println!("Stack Adress of s2 is : {:p}",&s2);
    println!("Heap Address of s2 is : {:p}",s2.as_ptr());
    println!("Heap length of s2 is : {}",s2.len());
    println!("Heap capacity of s2 is : {}",s2.capacity());

    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("????????????????????");
    println!("Stack Adress of s1 is : {:p}",&s1);
    println!("Heap Address of s1 is : {:p}",s1.as_ptr());
    println!("Heap length of s1 is : {}",s1.len());
    println!("Heap capacity of s1 is : {}",s1.capacity());
    println!("====================");

    println!("Stack Adress of s2 is : {:p}",&s2);
    println!("Heap Address of s2 is : {:p}",s2.as_ptr());
    println!("Heap length of s2 is : {}",s2.len());
    println!("Heap capacity of s2 is : {}",s2.capacity());
}

// pub fn run2(){
//     let x = 1;
//     println!("value of x in run2 is {}.",x);
// }