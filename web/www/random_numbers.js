export const NUM_COUNT = 6;
const RANDOM_BIG_NUMBER_PROBABILITY = 28;
const RANDOM_BIG_NUMBERS = [10, 25, 50, 75, 100];
export const MIN_NUMBER = 1;
export const MAX_NUMBER = 100;
export const MIN_TARGET = 100;
export const MAX_TARGET = 999;

export function random_int(min, max)
	{
	if (!Number.isInteger(min) || !Number.isInteger(max))
		throw new Error("random_int function only accepts integer numbers");

	// 1. Math.random() gives a float: [0.0, 1.0)
	// 2. We multiply by (max - min + 1) to cover the full range of numbers needed.
	// 3. Math.floor() cuts off the decimals.
	// 4. We add 'min' to shift the range start from 0 to 'min'.
	return Math.floor(Math.random() * (max - min + 1)) + min;
	}

export function generate_numbers()
	{
	let numbers = [];
	for (let i = 0; i < NUM_COUNT; i++)
		{
		const probability = random_int(0, 100);
		if (probability < RANDOM_BIG_NUMBER_PROBABILITY)
			{
			const big_number_pos = random_int(0, RANDOM_BIG_NUMBERS.length - 1);
			numbers.push(RANDOM_BIG_NUMBERS[big_number_pos]);
			}
		else
			numbers.push(random_int(1, 9));
		}
	return numbers;
	}
