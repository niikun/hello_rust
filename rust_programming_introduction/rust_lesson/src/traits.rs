trait Fruits{
    fn price(&self) -> u32;
}

struct Apple;
struct Banana;

impl Fruits for Apple{
    fn price(&self) -> u32{
        10
    }
}
impl Fruits for Banana{
    fn price(&self) ->u32{
        5
    }
}

trait Summary{
    fn summarize(&self)->String{
        String::from("(Read more...)")
    }
}

trait Message{
    fn message(&self)->String;
}

struct NewArticle{
    Headline:String,
    location:String,
    author:String,
}
impl Summary for NewArticle{
    // fn summarize(&self)->String{
    //     format!("{}, by {} ({})",self.Headline,self.author,self.location)
    // }
}

impl Message for NewArticle{
    fn message(&self)->String{
        format!("message, for {} ({})",self.author,self.location)
    }
}

struct Tweet{
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet{
    fn summarize(&self)-> String {
        format!("{}:{}",self.username,self.content)
    }
}

pub fn run(){
    // let apple = Apple;
    // let banana = Banana;
    // get_price(apple);
    // get_price(banana);
    let article =NewArticle{
        Headline:String::from("This is headline"),
        location:String::from("Osaka"),
        author:String::from("niikun"),
    };
    let tweet = Tweet{
        username:String::from("niikun"),
        content:String::from("This is tweet"),
        reply:false,
        retweet:false,
    };
    println!("{}",get_summary(tweet));
    println!("{}",get_summary(article));
    // notify(&article);

    // notify2(&article);
}

fn get_price<T:Fruits>(fruits:T) {
    println!("Price is :{}",fruits.price());
}

fn get_summary<T:Summary>(items:T)-> String {
    items.summarize()
}

// fn notify(item: &impl Summary)-> String {
//     println!("{}",item.summarize())
// }

// fn notify2(item: &(impl Summary + Message)) -> String {
//     println!("Summary: {}",item.summarize());
//     println!("Message! {}",item.message());
// }