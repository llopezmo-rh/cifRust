use wasm_bindgen::prelude::*;
use cifras_core::{SolutionStepStack, resolve_cifras as core_resolve, NUM_COUNT};

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub struct WasmStep
	{
	pub a: u64,
	pub b: u64,
	pub result: u64,
	pub op_char: char, 
	}

#[wasm_bindgen]
pub struct WasmSolution 
	{
	inner: SolutionStepStack,
	}

#[wasm_bindgen]
impl WasmSolution 
	{
	pub fn get_count(&self) -> usize 
		{
		return self.inner.count();
		}

	pub fn get_result(&self) -> u64 
		{
		// Si result devuelve Option, usamos unwrap_or para tener un valor por defecto
		return self.inner.result().unwrap_or(0);
		}

	pub fn get_step_object(&self, index: usize) -> WasmStep 
		{
		// CORRECCIÓN CRÍTICA:
		// Convertimos Option<&[SolutionStep]> en &[SolutionStep] (slice puro)
		// Si es None, usamos un slice vacío &[]
		let steps_slice = self.inner.get_steps().unwrap_or(&[]);

		if index >= steps_slice.len() 
			{
			// Retornamos un paso vacío si el índice se sale
			return WasmStep { a:0, b:0, result:0, op_char:'?' };
			}

		let step = steps_slice[index];

		// Lógica simple para detectar el operador basándonos en el string
		let step_str = step.to_string();
		let mut operator = '?';
		
		if step_str.contains('+') { operator = '+'; }
		else if step_str.contains('-') { operator = '-'; }
		else if step_str.contains('*') { operator = '*'; }
		else if step_str.contains('/') { operator = '/'; }

		return WasmStep
			{
			a: step.a,
			b: step.b,
			result: step.result,
			op_char: operator,
			};
		}
		
	pub fn get_step_string(&self, index: usize) -> String 
		{
		// CORRECCIÓN CRÍTICA AQUÍ TAMBIÉN:
		let steps = self.inner.get_steps().unwrap_or(&[]);
		
		if index >= steps.len() 
			{
			return String::new();
			}
		return steps[index].to_string();
		}
	}

#[wasm_bindgen]
pub fn resolve_cifras(numbers_slice: &[u64], target: u64) -> WasmSolution 
	{
	if numbers_slice.len() < NUM_COUNT 
		{
		return WasmSolution { inner: SolutionStepStack::new() };
		}

	let mut numbers_fixed: [u64; NUM_COUNT] = [0; NUM_COUNT];
	
	// Copiamos los datos de JS al array fijo de Rust
	numbers_fixed.copy_from_slice(&numbers_slice[..NUM_COUNT]);

	let result_stack = core_resolve(&numbers_fixed, target);

	return WasmSolution { inner: result_stack };
	}
