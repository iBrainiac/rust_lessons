enum Cultures<Y>{
    ONE(Y),
    TWO(i32),
    
}


fn main() {
    let one = Cultures::TWO(56);
    let two = Cultures::ONE("Hello world".to_string());

    match one {
        Cultures::ONE(value)=>println!("The value is {}", value),
        Cultures::TWO(value) => println!("Cluture is two {}", x)
    }

}