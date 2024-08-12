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

}