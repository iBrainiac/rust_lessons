// gen rand no b2n 1-10 & sort in descending order
use rand::Rng;

pub fn random(){
    //println!("random");
    let mut numbers = vec![];
for _ in 0..7 {
    numbers.push(rand::thread_rng().gen_range(1..=100));
}

// Prints generated numbers
println!("Generated numbers: {:?}", numbers);

// Sort in descending order
numbers.sort_by(|a, b| b.cmp(a)); 

// Prints sorted numbers
println!("Sorted in descending order: {:?}", numbers);
}

