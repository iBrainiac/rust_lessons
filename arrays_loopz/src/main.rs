fn main() {
    let array: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    println!("For in loop");
    for elem: char in array{
        println!("{elem}");

    }
    
    let mut idx: usize = 0;


    println!("LOOP");
    loop{
        println!{"{}", array[idx]};
        if idx == array.len() {
            break;
        }
        idx +=1;
    }
  
}