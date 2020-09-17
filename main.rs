extern crate rand;
use rand::Rng;
use std::io;
use std::time::{Instant};
fn main() {
    let mut input = vec![vec![2, 2, 1], vec![1, 3, 4],vec![5, 3, 3]];  
    let mut best_score = std::i32::MAX;
    let mut best_index = 0;
    let mut harmonic_memory = vec![vec![]; 3];
    let mut harmony_memory: Vec<i32> = Vec::new();

    //print original vector
    println!("Original Vector:");
    for i in 0..input.len(){
        println!("{:?}", input[i]);
    }
    println!();

    //user input for number of iterations
    println!("Please input the number of iterations");
    let mut num_of_iterations = String::new();
    io::stdin().read_line(&mut num_of_iterations).expect("Failed to read line");
    let num_of_iterations: usize = num_of_iterations.trim().parse().expect("Please type a number");
    
    let now = Instant::now();
    for _i in 0..num_of_iterations{
        let mut worst_score = std::i32::MIN;
        let mut worst_index = 0;
        //remove the worst score form input
        for j in 0..input.len(){
            println!();
            if equation(input[j][0], input[j][1], input[j][2]) > worst_score{
                worst_score = equation(input[j][0], input[j][1], input[j][2]);
                worst_index = j;
            }
            println!("Worst score {:?}", worst_score);
            println!("Worst index {:?}", worst_index);
        }
        input.remove(worst_index);

        //push the two best scores into harmony memory and best inputs into harmonic memory
        for j in 0..input.len(){
            println!();
            harmony_memory.push(equation(input[j][0], input[j][1], input[j][2]));
            for k in 0..input[j].len(){
                harmonic_memory[j].push(input[j][k]);
            }
            println!("Harmony memory {:?}", harmony_memory);
            println!("Harmonic memory {:?}", harmonic_memory);
        }

        println!();

        //pick a number from each column to create a new row in the input
        let mut vec : Vec<i32> = Vec::new();
        println!();
        let mut j = 0;
        while vec.len()!=input[0].len(){
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(0, input.len());
            println!("X is {} and J is {}", x, j);
            vec.push(input[x][j]);
            j+=1;
        }
        input.push(vec);

        //print the current input
        println!();
        println!("Current input: ");
        for i in 0..input.len(){
            println!("{:?}", input[i]);
        }
        println!();
        }

        //find the index of the best vector 
        for i in 0..harmony_memory.len(){
            if harmony_memory[i] < best_score{
                best_score = harmony_memory[i];
                best_index = i;
            }
        }
        
        //push values into harmonic
        let mut harmonic: Vec<i32>  = Vec::new();
        for i in 0..input[0].len(){
        harmonic.push(harmonic_memory[best_index][i]);
    }

    let time = now.elapsed().subsec_nanos();
    println!("The most optimized sequence in {} iterations is {:?}. It took {:?} nanoseconds to find this solution", num_of_iterations, harmonic, time);
}

fn equation(a: i32, b:i32, c:i32) -> i32{
    return (a-2)*(a-2) + (b-3)*(b-3)*(b-3)*(b-3) + (c-1)*(c-1) + 3;
}