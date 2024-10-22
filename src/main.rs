// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

fn main() {

    let mut value: u128 = 1;
    let mut num: u128 = 20;
    
    loop {
        
        if value % num != 0 {
            num = 20;
            value += 1;
            println!("{:?}", value);
        }
        else if num >= 20 {
            break;
        }
        num -= 1;
        println!("{:?}", value);
    }
    println!("| {:?} | is evenly divisible", value);

}
