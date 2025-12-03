import init, { resolve_cifras } from '../pkg/cifras_web.js';
import { generate_numbers, random_int, MIN_TARGET, MAX_TARGET, MIN_NUMBER, MAX_NUMBER } from './random_numbers.js';

const NUM_COUNT = 6;

window.onload=function()
    {
	load_wasm();
    populate_random_numbers();

	document.getElementById("calculate_button").addEventListener("click", calculate_cifras);
	document.getElementById("target_input").addEventListener("change", calculate_cifras);
	
	document.getElementById("randomize_button").addEventListener("click", populate_random_numbers);

	for (let i = 1; i <= NUM_COUNT; i++)
        {
        const input_id = `n${i}`;
        const input_element = document.getElementById(input_id);
        if (input_element)
            input_element.addEventListener("input", validate_single_input);
        }

    const target_element = document.getElementById("target_input");
    if (target_element)
        target_element.addEventListener("input", validate_single_input);
	
	document.getElementById("error_overlay").addEventListener("click", hide_error);
    document.getElementById("error_box").addEventListener("click", hide_error);
	}

async function load_wasm()
	{
	console.log("Loading Wasm...");
	await init();
    console.log("Wasm load successfully finished");
	}

function calculate_cifras()
	{
	console.log("Executing calculate_cifras...")
	
	const target_input = document.getElementById("target_input").value;
	if (!str_is_int(target_input))
		show_error(`${target_input} is not an integer number`);
	const target = BigInt(Number(target_input));
	if (target < MIN_TARGET || target > MAX_TARGET)
		show_error(`${target} is not a number between ${MIN_TARGET} and ${MAX_TARGET}`);
	
	let numbers = new BigUint64Array(NUM_COUNT);
	for (let i = 1; i <= NUM_COUNT; i++)
		{
		const n = document.getElementById(`n${i}`).value;
		if (!str_is_int(n))
			show_error(`${n} is not an integer number`);
		if (Number(n) < MIN_NUMBER || Number(n) > MAX_NUMBER)
			show_error(`${n} is not a number between ${MIN_NUMBER} and ${MAX_NUMBER}`);
		// Array starts in 0, therefore i-1
		numbers[i - 1] = BigInt(Number(n));
		}
	console.log("Numbers:", numbers);
	console.log("Target:", target);
	const solution_steps = resolve_cifras(numbers, target);
	const count = solution_steps.get_count();
	console.log("Result:", solution_steps.get_result());
	//document.getElementById("result").innerHTML = `Result: ${result}`;
	print_result(solution_steps, count, target);
	}

function print_result(solution_steps, count, target)
	{
	const result = solution_steps.get_result();
	let html = `Result: <b>${result} `;
	const diff = solution_steps.get_result() - target;
	if (diff == 0)
		{
		html += "(EXACT!)<br><br>";
		}
	else if (diff > 0)
		{
		html += `(+${diff})<br><br>`;
		}
	else
		{
		html += `(${diff})<br><br>`;
		}
	html += "</b>Steps:<br>";
	for (let i = 0; i < count; i++)
		{
		const step = solution_steps.get_step_object(i);
		html += `${step.a} ${step.op_char} ${step.b} = ${step.result}<br>`
		}
	//document.getElementById("result").innerHTML = html;
	const div = document.getElementById("result_div");
	div.innerHTML = html;
	//div.style.opacity = "0.2";
	if (diff == 0)
		{
		div.style.backgroundColor = "green";
  		div.style.border = "5px solid #4CAF50";
		}
	else
		{
		div.style.backgroundColor = "#e84a5f";
  		div.style.border = "5px solid red";
		}
	}

function str_is_int(str)
	{
	if (typeof str != 'string')
		throw new Error("str_is_int function only accepts strings");
	if (Number.isInteger(Number(str)))
		{
		console.log(str + " is an integer");
		return true;
		}
	else
		{
		console.log(str + " is NOT an integer");
		return false;
		}
	}

function validate_single_input(event)
    {
    // The element that triggered the event
    const input_element = event.target;
    const val_str = input_element.value;
    const input_id = input_element.id;
    let is_valid = true;

    // --- LOGIC TO DETERMINE VALIDITY ---

    // Case 1: Target Input
    if (input_id === "target_input")
        {
        // Empty is considered temporarily valid while typing
        if (val_str.trim() !== "")
            {
            const val_num = Number(val_str);
            if (isNaN(val_num) || !Number.isInteger(val_num) || val_num < MIN_TARGET || val_num > MAX_TARGET)
                {
                is_valid = false;
                }
            }
        }
    // Case 2: Number Inputs (n1-n6)
    else if (input_id.startsWith("n"))
        {
        if (val_str.trim() !== "")
            {
            const val_num = Number(val_str);
            if (isNaN(val_num) || !Number.isInteger(val_num) || val_num < MIN_NUMBER || val_num > MAX_NUMBER)
                {
                is_valid = false;
                }
            }
        }

    // --- VISUAL FEEDBACK ---

    if (is_valid)
        {
        // If valid, remove error styles (reset to default)
        input_element.style.borderColor = "";
        input_element.style.backgroundColor = "";
        }
    else
        {
        // If invalid, set error styles (red border and light red background)
        input_element.style.borderColor = "#d32f2f";
        input_element.style.backgroundColor = "#ffebee";
        }
    }

function populate_random_numbers()
	{
	console.log("Calculate random numbers");
	const numbers = generate_numbers();
	const target = random_int(MIN_TARGET, MAX_TARGET);

	console.log("Populate random numbers");
	for (let i = 0; i < NUM_COUNT; i++)
		document.getElementById(`n${i + 1}`).value = numbers[i];
	document.getElementById("target_input").value = target;

	console.log("Clean result div");
	document.getElementById("result_div").innerHTML = "";
	document.getElementById("result_div").style.backgroundColor = "white";
	document.getElementById("result_div").style.border = "none";
	}

function show_error(message)
	{
    document.getElementById("error_message_text").textContent = message;
    document.getElementById("error_overlay").style.display = 'block';
    document.getElementById("error_box").style.display = 'block';
	}

function hide_error()
	{
    document.getElementById("error_overlay").style.display = 'none';;
    document.getElementById("error_box").style.display = 'none';
	}
