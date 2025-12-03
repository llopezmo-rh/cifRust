use std::cmp::Ordering;


pub const NUM_COUNT: usize = 6;
pub const MAX_SOLUTION_STEPS: usize = if NUM_COUNT > 4 
		{ 
		NUM_COUNT - 1 
		} 
	else 
		{ 
		4 
		};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operation
	{
	Add,
	Sub,
	Mul,
	Div,
	None // Empty/invalid slot
	}
impl std::fmt::Display for Operation
	{
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
		{
		let symbol = match self
			{
			Operation::Add => '+',
			Operation::Sub => '-',
			Operation::Mul => '*',
			Operation::Div => '/',
			Operation::None => '?',
			};
		return write!(f, "{}", symbol);
		}
	}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SolutionStep
	{
	pub result: u64,
	pub a: u64,
	pub b: u64,
	pub op: Operation,
	}
impl std::fmt::Display for SolutionStep
	{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
        {
		return write!(f, "{} {} {} = {}", self.a, self.op, self.b, self.result);
		}
	}
impl SolutionStep
	{
	/// Return dummy values because Rust forces to initialize everything
	#[inline]
	pub fn empty() -> SolutionStep
		{
		return SolutionStep
			{
			result: 0,
			a: 0,
			b: 0,
			op: Operation::None,
			}
		}
	}

/// The fields are private to force the use of the ADT functions for its manipulation
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SolutionStepStack
	{
    steps: [SolutionStep; MAX_SOLUTION_STEPS],
    count: usize,
    }
impl SolutionStepStack
    {
    #[inline]
    pub fn new() -> SolutionStepStack
        {
        // SolutionStep, as well as all the values, must be initilized
        let dummy = SolutionStep::empty();
        return SolutionStepStack
            {
            steps: [dummy; MAX_SOLUTION_STEPS],
            count: 0,
            }
        }

	 #[inline]
	 pub fn is_empty(&self) -> bool
		{
		return self.count == 0;
		}

	#[inline]
	pub fn count(&self) -> usize
		{
		return self.count;
		}

	#[inline]
	pub fn result(&self) -> Option<u64>
		{
		if self.count == 0
			{
			return None;
			}
		return Some(self.steps[self.count - 1].result);
		}

	#[inline]
	pub fn push(&mut self, step: &SolutionStep) -> Result<(), &'static str>
		{
		if self.count >= MAX_SOLUTION_STEPS
			{
			return Err("Error in SolutionStepStack push: stack full. Overflow\n");
			}
		self.steps[self.count] = *step;
		self.count += 1;
		return Ok(());
		}

	#[inline]
	pub fn pop(&mut self) -> Result<SolutionStep, &'static str>
		{
		if self.count == 0
			{
			return Err("Error in SolutionStepStack pop: stack empty\n");
			}
		self.count -= 1;
		return Ok(self.steps[self.count]);
		}
	
	#[inline]
	pub fn get_steps(&self) -> Option<&[SolutionStep]>
		{
		if self.count == 0
			{
			return None;
			}
		else
			{
			return Some(&self.steps[0..self.count]);
			}
		}
	
	/// A solution is better if its result is nearer the target and, in case
	/// the distance to the target is the same, a solution is better if it is
	/// shorter (less steps)
	pub fn compare(&self, other: &SolutionStepStack, target: u64 ) -> Ordering
		{
		match (self.is_empty(), other.is_empty())
			{
			(true, true) => return Ordering::Equal,
			(true, false) => return Ordering::Greater,
			(false, true) => return Ordering::Less,
			(false, false) => {},
			}
		// The method abs_diff is in the standard library
		let diff_self: u64 = self.result().unwrap().abs_diff(target);
		let diff_other: u64 = other.result().unwrap().abs_diff(target);
		// cmp is in the trait Ord and can compare many data types, not only
		// integer numbers. Therefore, it requires a reference.
		//
		// "then" is executed only if the result of cmp is Equal
		return diff_self.cmp(&diff_other).then(self.count().cmp(&other.count()));
		}
	}

fn build_candidates_stack(op1: u64, op2: u64) -> SolutionStepStack
	{
	let mut stack: SolutionStepStack = SolutionStepStack::new();
	
	assert!(op1 > 0 && op2 > 0);

	// Addition
	stack.push(&SolutionStep {
		a: op1, b: op2, result: op1 + op2, op: Operation::Add
		}).unwrap();

	// Susbtraction
	// No push if op1 == op2
	if op1 > op2
		{
		stack.push(&SolutionStep {
			a: op1, b: op2, result: op1 - op2, op: Operation::Sub
			}).unwrap();
		}
	else if op1 < op2
		{
		stack.push(&SolutionStep {
			a: op2, b: op1, result: op2 - op1, op: Operation::Sub
			}).unwrap();
		}

	// Prune if any of the operands is 1. Either multiplying or dividing by 1
    // introduces a useless operation.
    //
    // A recursive branch with a useless operation will never be the best
    // because there will be always, at least, one branch with the same result
    // but shorter (with less operations).
	if op1 == 1 || op2 == 1
		{
		return stack;
		}

	// Product
	stack.push(&SolutionStep {
		a: op1, b: op2, result: op1 * op2, op: Operation::Mul
		}).unwrap();
	
	// Divide
    //
    // Optimization maximized. It is the most CPU-expensive operation.
    //
    // If dividend == divisor, set the result directly to 1 instead of
    // executing the division in order to save CPU cycles
	if op1 == op2
		{
		stack.push(&SolutionStep {
			a: op1, b: op2, result: 1, op: Operation::Div
		}).unwrap();
		}
	// The operands are compared before the modulo operation. That saves
    // one modulo operation (1 vs 2) if operand1 != operand2
	else if op1 > op2 && op1 % op2 == 0
		{
		stack.push(&SolutionStep {
			a: op1, b: op2, result: op1 / op2, op: Operation::Div
			}).unwrap();
		}
	else if op2 > op1 && op2 % op1 == 0
		{
		stack.push(&SolutionStep {
			a: op2, b: op1, result: op2 / op1, op: Operation::Div
			}).unwrap();
		}

	return stack;
	}

/// 1. Put new in `next_numbers[0]`.
/// 2. Copy the elements of `former_numbers` into `next_numbers` starting from
/// `next_numbers[1]` and skiping `former_numbers[pos1]` and `former_numbers[pos2]`
fn build_next_numbers(former_numbers: &[u64; NUM_COUNT],  pos1: usize,
	pos2: usize, new: u64) -> [u64; NUM_COUNT]
	{
	// Initialize result
	let mut next_numbers: [u64; NUM_COUNT] = [0; NUM_COUNT];
	next_numbers[0] = new;
	
	// Fill out the array
	let mut j: usize = 1;
	for i in 0..NUM_COUNT
		{
		if i != pos1 && i != pos2
			{
			next_numbers[j] = former_numbers[i];
			j += 1;
			}
		}
	return next_numbers;
	}

/// Return `true` if the exact number has been already found and therefore a
/// solution with more steps can never be better
#[inline]
fn prunable_length(current_steps: &SolutionStepStack,
	best_steps: &SolutionStepStack, target: u64) -> bool 
	{
	if current_steps.is_empty()
		{
		return false;
		}
	assert!(!best_steps.is_empty(), "If current_steps is not empty, best_steps \
		should never be empty");
	
	if best_steps.result().unwrap() != target
		{
		return false;
		}

	if current_steps.count() < best_steps.count()
		{	
		return false;
		}

	return true
	}


/// Calculation of an additional prune.
/// `true` if all the following conditions are satisfied:
/// 1. `best_steps` is not empty.
/// 2. The highest value obtained by combining the pending numbers is smaller
/// than the target.
/// 3. The highest value obtained by combining the pending numbers is further
/// from the target than the result of `best_steps`.
fn prunable_upper_value(numbers: &[u64; NUM_COUNT], numbers_count: usize,
	target: u64, best_steps: &SolutionStepStack) -> bool
	{
	let mut upper_value: u64 = 1;
	
	// No prune if best_steps is empty
	if best_steps.is_empty()
		{
		return false;
		}

	for i in 0..numbers_count
		{
		// Upper bound estimate: multiplying by 2 will never reach a value
		// smaller than any other one combining the number 1
		if numbers[i] == 1
			{
			upper_value *= 2;
			}
		else
			{
			upper_value *= numbers[i];
			}

		// No prune if upper_value is larger than target
		if upper_value > target
			{
			return false;
			}
		}

	let upper_value_diff: u64 = target - upper_value;
	let best_diff: u64 = best_steps.result().unwrap().abs_diff(target);
	return upper_value_diff > best_diff;
	}

/// Backtracking function
fn cifras_bt(numbers: &[u64; NUM_COUNT], numbers_count: usize, target: u64,
	current_steps: &SolutionStepStack, best_steps: &mut SolutionStepStack)
	{
	// If current_steps reaches a better result than best_steps, then
	// mirror current_steps into best_steps
	if current_steps.compare(best_steps, target) == Ordering::Less
		{
		*best_steps = *current_steps;
		}
	
	// Base cases:
	// 1. Only 1 number pending, therefore no more combinations are possible
	assert!(numbers_count > 0);
	if numbers_count == 1
		{
		return;
		}
	// 2. Prune if exact has been already found and the current steps count
	// is higher than the exact solution
	if prunable_length(current_steps, best_steps, target)
		{
		return;
		}
	// 3. Prune is the upper value obtained by combining all the pending
	// numbers is smaller than the target AND is further from the target than
	// the result of best_steps
	if prunable_upper_value(numbers, numbers_count, target, best_steps)
		{
		return;
		}

	// From here onwards, recursive case
	for i in 0..numbers_count
		{
		for j in (i + 1)..numbers_count
			{
			// Stack candidate steps
			// Operands: numbers[i] and numbers[j]
			let mut candidate_steps: SolutionStepStack = build_candidates_stack(
				numbers[i], numbers[j]);

			// Initialize next_steps with a copy of current_steps
			let mut next_steps: SolutionStepStack = *current_steps;

			// Pop candidates one by one and make a recursive call with
			// everyone of them
			while !candidate_steps.is_empty()
				{
				// Initialize next_steps with a copy of current_steps
				//let mut next_steps: SolutionStepStack = current_steps;
				
				// Push next candidate
				let candidate: SolutionStep = candidate_steps.pop().unwrap();
				next_steps.push(&candidate).unwrap();

				// Create numbers array for the recursive call
				let next_numbers: [u64; NUM_COUNT] = build_next_numbers(numbers,
					i, j, candidate.result);

				// Recursive call
				cifras_bt(&next_numbers, numbers_count - 1, target, &next_steps,
					best_steps);

				// Restore next_steps. More than one candidate step must not
				// be pushed for the same recursive call
				next_steps.pop().unwrap();
				}
			}
		}
	}

/// Public wrapper for cifras_bt
pub fn resolve_cifras(numbers: &[u64; NUM_COUNT], target: u64) -> SolutionStepStack
	{
	let current_steps: SolutionStepStack = SolutionStepStack::new();
	let mut best_steps: SolutionStepStack = SolutionStepStack::new();
	cifras_bt(numbers, NUM_COUNT, target, &current_steps, &mut best_steps);
	return best_steps;
	}
