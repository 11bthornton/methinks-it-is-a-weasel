#![feature(array_zip)]

use rand::{
	Rng,
	prelude::SliceRandom
};

use std::env;

const TARGET : &[u8] = "Good night Sena, I got it working! :)".as_bytes();

#[derive(Debug)]
pub struct Individual {
	chars: [u8; 38]
}

fn demo<T>(v: Vec<T>) -> [T; 38] where T: Copy {
    let slice = v.as_slice();
    let array: [T; 38] = match slice.try_into() {
        Ok(ba) => ba,
        Err(_) => panic!("Expected a Vec of length {} but it was {}", 38, v.len()),
    };
    array
}

impl Individual {

	pub fn new() -> Self {
		let mut rng = rand::thread_rng();

    	let vals: Vec<u8> = (0..38)
    		.map(|_| rng.gen_range(32..127))
    		.collect();

		Individual {
			chars: demo(vals)
		}

	}

	pub fn fitness(&self) -> u8 {

		let mut fitness : u8 = 0;

		for i in 0..37 {
			if self.chars[i] == TARGET[i] {
				fitness += 1;
			}
		}

		fitness
	}

	pub fn as_str(&self) -> &str {
		std::str::from_utf8(&self.chars).unwrap()
	}

	pub fn mutate(&self, chance: f32) -> Self {

		let mut rng = rand::thread_rng();

		let mut chars : [u8; 38] = [0; 38];

		for i in 0..37 {

			let flag : usize = rng.gen_range(1..10000);

			if (flag as f32) <= (10000 as f32)*chance {

				let ff : u8 = rng.gen_range(32..127);
				chars[i] = ff;

			} else {

				chars[i] = self.chars[i];
			}
		}

		Individual {
			chars: chars
		}
	}

	pub fn crossover(parent_1: &Self, parent_2: &Self) -> Individual {
		let mut chars : [u8; 38] = [0; 38];

		for i in 0..37 {
			if rand::random() {
				chars[i] = parent_1.chars[i];
			} else {
				chars[i] = parent_2.chars[i];
			}
		}

		Individual {
			chars: chars
		}
	}
}

#[allow(dead_code)]
fn hill_climb_test() {
	let mut individual : Individual = Individual::new();

	let mut count = 0;

	while individual.fitness() < 37 {
		let mutated = individual.mutate(0.03);

		if mutated.fitness() > individual.fitness() {
			count+=1;
			individual = mutated;
			println!("{}", individual.as_str());
		}
	}

	println!("{:?}", count);
}

fn main() {

	let args: Vec<String> = env::args().collect();

	let pop_size : usize = args[1].parse().unwrap();
	let mutation_chance : f32 = args[2].parse().unwrap();

	// Generate 500 population examples 
	let mut population : Vec<Individual> = (0..pop_size)
		.map(|_| Individual::new())
		.collect(); 

	// Set fitness to 0
	let mut fitness = 0; 

	let mut rng = rand::thread_rng();

	while fitness < 37 {

		let pop_a : &Individual = population.choose(&mut rand::thread_rng())
			.unwrap();

		let pop_b : &Individual = population.choose(&mut rand::thread_rng())
			.unwrap();

		let parent_1; 

		if pop_a.fitness() > pop_b.fitness() {
			parent_1 = pop_a;
		} else {
			parent_1 = pop_b;
		}

		let pop_a : &Individual = population.choose(&mut rand::thread_rng())
			.unwrap();

		let pop_b : &Individual = population.choose(&mut rand::thread_rng())
			.unwrap();

		let parent_2; 

		if pop_a.fitness() > pop_b.fitness() {
			parent_2 = pop_a;
		} else {
			parent_2 = pop_b;
		}

		let crossover : Individual = Individual::crossover(&parent_1, &parent_2);
		let child : Individual = crossover.mutate(mutation_chance);

		let ia : usize = rng.gen_range(0..pop_size);
		let ib : usize = rng.gen_range(0..pop_size);

		if population[ia].fitness() > population[ib].fitness() {
			population[ib] = child;
		} else {
			population[ia] = child;
		}

		let mut ba = "";
		let prev_fit = fitness;
		for individual in &population {
			if individual.fitness() >= fitness {
				fitness = individual.fitness();
				ba = individual.as_str();
			}
		}

		if prev_fit != fitness {
			println!("Best: {} :: Fitness {}", ba, fitness);
		}
		
	}

}