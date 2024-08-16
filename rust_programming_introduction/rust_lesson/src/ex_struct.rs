struct Point<T>{
    x:T,
    y:T,
}

struct PointAnother<T,U>{
    x:T,
    y:U,
}
impl<T,U> PointAnother<T,U>{
    fn mixup<V,W>(self,other:PointAnother<V,W>)->PointAnother<T,W>{
        PointAnother{
            x:self.x,
            y:other.y,
        }
    }
}

pub fn run(){
    let p1 = Point{x:1,y:2};
    let p2 = Point{x:1.0,y:2.0};
    let p3 = PointAnother{x:'a',y:'b'};
    let p4 = PointAnother{x:1.5,y:"name"};

    println!("Point p1 is at ({},{})",p1.x,p1.y);
    println!("Point p2 is at ({},{})",p2.x,p2.y);
    println!("Point p3 is at ({},{})",p3.x,p3.y);
    println!("Point p4 is at ({},{})",p4.x,p4.y);
    let p5 = p3.mixup(p4);
    println!("Point p5 is at ({},{})",p5.x,p5.y);
}