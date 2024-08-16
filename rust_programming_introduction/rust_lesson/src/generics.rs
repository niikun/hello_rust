pub fn run(){
    let num_list = vec![34,50,25,100,65];
    let result = largest(num_list);
    // let mut largest = num_list[0];
    // for num in num_list{
    //     if num > largest {
    //         largest = num;
    //     }
    // }
    println!("The largest number is {}",result);

    //char example
    let char_list = vec!['a','b','c','d'];
    println!("the largest char is {}",largest(char_list));

}

fn largest<T:PartialOrd + Copy>(list: Vec<T>) -> T{
    let mut largest = list[0];
    for item in list{
        if item > largest {
            largest = item;
        }
    }
    largest
}

// fn large_char(list: Vec<char>)->char{
//     let mut largest = list[0];
//     for i in list{
//         if i > largest{
//             largest=i;
//         }
//     }
//     largest
// }