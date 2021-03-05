use std::fs;

fn does_match(numbers: &Vec<i32>, indexes: &Vec<usize>, sum: i32) -> bool{
    let mut value: i32 = 0;
    for x in 0..indexes.len() {
        let x = x as usize;
        value += numbers[indexes[x]];
    }
    return value == sum;
}

fn get_result(numbers: &Vec<i32>, indexes: &Vec<usize>) -> i32{
    let mut result: i32 = 1;
    for x in 0..indexes.len(){
        let x = x as usize;
        result *= numbers[indexes[x]];
    }
    return result;
}

fn calculate(numbers: &Vec<i32>, count: i32, sum: i32) -> i32{
    let mut indexes: Vec<usize> = Vec::<usize>::new();
    for _ in 0..count{
        indexes.push(0);
    }
    while !does_match(&numbers, &indexes, sum) {
        for x in 0..(indexes.len()){
            if indexes[x] < numbers.len() - 1 {
               continue; 
            }
            indexes[x]=0;
            let mut next = x+1;
            if next == indexes.len(){
                next = 0;
            }
            indexes[next]+=1;
        }   
        indexes[0]+=1;
    }
    return get_result(&numbers, &indexes);
}

fn main() {
    let data = fs::read_to_string("../../input.txt").expect("Unable to read file");
    let numbers: Vec<i32> = data.split("\r\n").map(|x| x.parse::<i32>().unwrap()).collect();
    println!("One: {:?}",calculate(&numbers, 2, 2020));
    println!("Two: {:?}",calculate(&numbers, 3, 2020));
}

