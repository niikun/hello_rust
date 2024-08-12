enum List{
    Node(i32, Box<List>),
    Nil
}

pub fn run(){
    // // println!("this is stack_heap module");
    // let a1 :[u8; 5000000] = [0; 5000000];
    // println!("{:?}", arr);
    let mut v1 = vec![1,2,3,4,5];

    println!("Stack address is : {:p}",&v1);
    println!("Heap Address is {:?}",v1.as_ptr());
    println!("Heap length is :{}",v1.len());
    println!("Heap capacity is :{}",v1.capacity());

    v1.insert(1,10);
    println!("Stack address is : {:p}",&v1);
    println!("Heap Address is {:?}",v1.as_ptr());
    println!("Heap length is :{}",v1.len());
    println!("Heap capacity is :{}",v1.capacity());

    let t1:(i64,String) = (10,String::from("hello!"));
    println!("Stack Address of t1 is :{:p}",&t1);
    println!("Heap address of t1,1 is :{:?}",t1.1.as_ptr());
    println!("Heap length of t1.1 is :{}",t1.1.len());
    println!("Heap capacity of t1.1 is :{}",t1.1.capacity());

    let mut b1 = Box::new(t1);
    (*b1).1 += " world";
    println!("{}{}",b1.0, b1.1);
    println!("Stack asdress of b1 is :{:p}",&b1);
    println!("Heap address of bi.1 is :{:?}",b1.1.as_ptr());
    println!("Heap length of b1.1 is :{}",b1.1.len());
    println!("Heap capacity of b1.1 is :{}",b1.1.capacity());

}