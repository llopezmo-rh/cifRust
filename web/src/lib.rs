use wasm_bindgen::prelude::*;
use cifras_core::{SolutionStepStack, resolve_cifras as core_resolve};


#[wasm_bindgen]
pub fn resolve_cifras(numbers_slice: &[u64], target: u64) -> SolutionStepStack
	{
	// Convert slice to fixed array for internal logic
	let mut numbers: [u64; NUM_COUNT] = [0; NUM_COUNT];
	for (i, &val) in numbers_slice.iter().take(NUM_COUNT).enumerate()
		{
		numbers[i] = val;
		}

	let current_steps: SolutionStepStack = SolutionStepStack::new();
	let mut best_steps: SolutionStepStack = SolutionStepStack::new();
	
	cifras_bt(&numbers, NUM_COUNT, target, &current_steps, &mut best_steps);
	
	return best_steps;
	}
