enum OS{
    Windows(u32,String),
    Mac(u32,String),
    Linux(u32,String),
}


pub fn run(){
    let linux = OS::Linux(1991,String::from("Linus"));
    print_os_info(linux);
    let windows = OS::Windows(1985,String::from("Microsoft"));
    print_os_info(windows);
    let mac = OS::Mac(2001,String::from("Apple"));
}

fn print_os_info(os:OS){
    match os{
        OS::Windows(year,company)=>{
            println!("Windows : year :{}, company:{}",year,company);
        }
        OS::Mac(year,company)=>{
            println!("Mac : year :{}, company:{}",year,company);
        }
        OS::Linux(year,company)=>{
            println!("Linux : year :{}, company:{}",year,company);
        }
    }
}