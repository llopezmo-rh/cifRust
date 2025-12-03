//mod cifras_bt;
//use cifras_core::{SolutionStepStack, SolutionStep, NUM_COUNT};
use cifras_core::{SolutionStepStack, NUM_COUNT};

use colored::*;
use rand::Rng;
use std::io;
use std::io::Write;
//use std::process;


// Probability 0-100
const RANDOM_BIG_NUMBER_PROBABILITY: u8 = 28;
const RANDOM_BIG_NUMBERS: [u64; 5] = [10, 25, 50, 75, 100];
const MIN_NUMBER: u64 = 1;
const MAX_NUMBER: u64 = 100;
const MIN_TARGET: u64 = 100;
const MAX_TARGET: u64 = 999;


fn generate_numbers<R: Rng>(rng: &mut R) -> [u64; NUM_COUNT]
	{
	let mut numbers: [u64; NUM_COUNT] = [0; NUM_COUNT];
	for i in 0..NUM_COUNT
		{
		let probability: u8 = rng.gen_range(0..=100);

		if probability <= RANDOM_BIG_NUMBER_PROBABILITY
			{
			//let big_number_pos: usize = random_u16(rng, 0, (RANDOM_BIG_NUMBERS.len() - 1) as u16) as usize;
			let big_number_pos: usize = rng.gen_range(0..RANDOM_BIG_NUMBERS.len());
			numbers[i] = RANDOM_BIG_NUMBERS[big_number_pos];
			}
		else
			{
			//numbers[i] = random_u16(rng, 1, 9) as u64;
			numbers[i] = rng.gen_range(1..=9);
			}
		}
	return numbers;
	}

fn generate_data() -> ([u64; NUM_COUNT],u64)
	{
	let mut rng = rand::thread_rng();

	let numbers: [u64; NUM_COUNT] = generate_numbers(&mut rng);
	let target: u64 = rng.gen_range(100..=999);
	return (numbers, target);
	}

fn get_user_input(prompt: &str) -> String
	{
	let mut buffer = String::new();

	print!("{}", prompt);

	// Rust does not allow to set the buffer globally like setbuf in C.
	// It has to be flushed manually every time it is wanted to read from 
	// stdin without a new line
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut buffer).expect("Read error");
	
	// buffer.trim() returns a &str pointing to a subset of buffer. It is a
	// kind of "modified view" of buffer. It is not a String type. buffer 
	// will be "freed" automatically by the compiler once the function 
	// finishes. Therefore, buffer.trim() would return a reference to a 
	// non-existing String. To create a new String variable	from scratch and 
	// return it, we need to_string()
	return buffer.trim().to_string();
	}

fn parse_numbers(input: &str) -> Result<[u64; NUM_COUNT], String>
	{
	// split returns an iterator type "Split", similar to a vector, of strings.
	// If it finds ',' or ' ', it inserts an empty string in the Split.
	// Return example:
	// "10 , 20" -> ["10", "", "", "", "20"]
	// 
	// filter returns another iterator type "Filter". It orders a string and,
	// if it is empty, it orders the next one.
	// 
	// collect returns a vector with the strings provided by filter.
	let tokens: Vec<&str> = input
		.split(|c| c == ',' || c == ' ')
		.filter(|s| !s.is_empty())
		.collect();

	if tokens.len() != NUM_COUNT 
		{
		return Err(format!("❌ Error: {} numbers expected. Received: {}.",
			NUM_COUNT, tokens.len()));
		}
	
	let mut numbers: [u64; NUM_COUNT] = [0; NUM_COUNT];
	for (i, token) in tokens.iter().enumerate() 
		{
		// Parsing instead of '?' in order to customize the error message
		match token.parse::<u64>()
			{
			Ok(num) =>
				{
				if num < MIN_NUMBER || num > MAX_NUMBER
					{
					return Err(format!("❌ Error: \
						number {} not between {} and {}", 
						num, MIN_NUMBER, MAX_NUMBER));
					}
				numbers[i] = num;
				},
			Err(_) => return Err(format!("❌ Error: \
				'{}' is not a valid number", token)),
			}
		}
	return Ok(numbers);
	}

fn parse_target(input: &str) -> Result<u64, String>
	{
	match input.parse::<u64>()
		{
		Ok(num) =>
			{
			if num < MIN_TARGET || num > MAX_TARGET
				{
				return Err(format!("❌ Error: target {} not between {} and {}",
					num, MIN_TARGET, MAX_TARGET));
				}
			return Ok(num);
			},
		Err(_) => return Err(format!("❌ Error: '{}' is not a \
			valid target number", input)),
		};
	}

fn print_steps(stack: &SolutionStepStack)
	{
	for step in stack.get_steps().unwrap()
		{
		println!("{}", step);
		}
	}

fn get_data() -> ([u64; NUM_COUNT],u64)
	{
	// Get numbers from user
	let numbers: [u64; NUM_COUNT] = loop 
		{
		let input: String = get_user_input("Introduce numbers (ENTER for randomly \
			generated):\n");
		if input.trim().is_empty()
			{
			return generate_data();
			}
		match parse_numbers(&input)
			{
			Ok(numbers) => break numbers,
			Err(e) => println!("{}", e),
			};
		};

	// Get target from user
	let target: u64 = loop
		{
		let input = get_user_input("Target: ");
		match parse_target(&input)
			{
			Ok(num) => break num,
			Err(e) => println!("{}", e),
			};
		};

	return (numbers, target);
	}

fn print_result(best_steps: &SolutionStepStack,	target: u64)
	{
	let result: u64 = best_steps.result().unwrap();
	print!("{} {}", "Result obtained:".bright_blue().bold(), result);
	//let diff: i64 = (result - target).try_into().unwrap();
	let diff: i64 = result as i64 - target as i64;
	if diff == 0
		{
		println!("{}", " (EXACT! 🎯)".bold().bright_green());
		}
	else
		{
		println!(" {}{}{}",
			"(".yellow().bold(),
			format!("{:+}", diff).yellow().bold(),
			")".yellow().bold());
		}
	
	print_steps(&best_steps);
	}

/// Read a character without echo and without later ENTER needed
fn get_char(prompt: &str) -> std::io::Result<char> 
	{
	print!("{}", prompt);
	
	io::stdout().flush().unwrap();
    let term = console::Term::stdout();
    term.read_char()
	}

fn print_banner()
	{
	println!();
	println!("{}", "==================================".bright_blue());
	println!("{}", "      CIFRAS - NUMBERS GAME       ".bright_blue().bold());
	println!("{}", "==================================\n\n".bright_blue());
	}

fn main()
	{
	print_banner();
	
	loop
		{
		let (numbers, target) = get_data();
	
		println!("\n{}\n{:?}\n", "Numbers:".bright_blue().bold(),  numbers);
		println!("{} {}\n", "Target:".bright_blue().bold(), target);
	
		let best_steps = cifras_core::resolve_cifras(&numbers, target);
		print_result(&best_steps, target);
		let continue_char: char = get_char("\nPress Q for finishing or any \
			other key to play again...").unwrap();
		if continue_char == 'Q' || continue_char == 'q'
			{
			println!("");
			break
			}
		else
			{
			println!("\n\n");
			}
		}
	}

