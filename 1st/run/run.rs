
// use std::env;
use std::fs;


fn calculate(numbers: Vec<i32>, amount: i32){
    // let mut dict: HashMap<usize, usize> = HashMap::<usize, usize>::new();
    let mut dict: Vec<i32> = Vec::<i32>::new();
    for a in 0..amount{
        dict.push(0);
    }
    let mut value: i32 = 0;
    for x in 0..dict.len(){
        println!("{:?}{:?}", x, value);
        // value += numbers[dict[x]];
    }
    // for (index, value) in dict{
    //     println!("{:?}{:?}", index, value);
    //     value += numbers[&dict[&index]];
    // }
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");
    let numbers: Vec<i32> = data.split("\n").map(|x| x.parse::<i32>().unwrap()).collect();
    println!("calculating data");
    calculate(numbers.to_vec(), 2);
    calculate(numbers.to_vec(), 3);
    
}

