struct Point<0>{
    x:D,
    y:D
}
impl<D> Point<D,X,R>{
    fn new(x:D, y:X, z:R)->Self{
        Point{x,y,z}
    }
    fn get_x(&self)->&D{
        &self.x
    }
}
fn main(){
    // let d = Point{x:1,y:2};
    // let d =  Point{x:1.4,y:2.2};
    let my_new_one: Point<f64> = Point::new(x:3.5, y:5.6, z:1);
    Println!("x is at position {}", my_new_one.get_())
}