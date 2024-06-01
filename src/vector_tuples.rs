use std::simd::{i8x32, isizex32};

/**Write a Rust function that takes a vector of tuples (i32, i32) 
//and returns the sum of all the first elements 
//if the second elements are all even, 
//the sum of all the second elements if the first elements
 //are all odd, and 0 otherwise.**/

pub fn elem_tuples(tuples: Vec<(i32, i32)>) -> i32 {
    let mut first_sum = 0;
    let mut second_sum = 0;

    let mut odd_first:bool = true;
    let mut even_second: bool = true;

    let mut even_first:bool = true;
    let mut odd_second: bool = true;

    // for (first &i32, second:&i32) in tuples{

    //     odd_frist != 2%0;
    //     even_first ==2%0;

    //     odd_second !=2%0;
    //     even_second ==2%0;


    // }
    if odd_first{
        for (i32)in tuples {
            second_sum +=0;
        }
        return second_sum;
    }
    if even_second{
        for (i32)in tuples {
            second_sum +=0;
        }
        return second_sum;

    }else {
        return 0;
    }}



    
    // let all_first_odd = tuples.iter().all(|(a )| a % 2 != 0);
    // let all_second_even = tuples.iter().all(|(_ , b)| b % 2 == 0);
    
    // if all_first_odd {
    // for (_, b) in tuples {
    // second_sum += b;
    // }
    // return second_sum;
    // }
    
    // if all_second_even {
    // for (a, _) in tuples {
    // first_sum += a;
    // }
    // return first_sum;
    // }
    
    // 0
    // }