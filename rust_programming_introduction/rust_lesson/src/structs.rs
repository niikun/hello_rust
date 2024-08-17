#[derive(Debug)]
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height:u32,
}
impl Rectangle{
    fn create(width:u32, height:u32)->Self{
        Self{ width, height }
    }
    fn area(&self){
        let res = self.width * self.height;
        println!("area :{}",res);
    }
}


pub fn run(){

    let rec1 = Rectangle::create(30, 50);
    println!("rec1 :{:#?}",rec1); 
    rec1.area();

    // let user1 = User{
    //     email: String::from("niikun@example.com"),
    //     username: String::from("niikun"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // let user2 = user1;
    // println!("user1 email: {:#?}", user2);

//     let mut user1 = User{
//         email: String::from("niikun@example.com"),
//         username: String::from("niikun"),
//         active: true,
//         sign_in_count: 1,
//     };
//     user1.email = String::from("niikun@another.com");
//     println!("user1 : {:#?}", user1);
//     let user2 = build_user(String::from("sankun@email.com"), String::from("sankun"));
//     println!("user2 : {:#?}", user2);
// }

// fn build_user(email:String, username:String)->User{
//     User{
//         email,
//         username,
//         active: true,
//         sign_in_count:1,
//     }
}