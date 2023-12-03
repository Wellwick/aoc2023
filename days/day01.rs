use std::fs;

fn get_first_and_last_digit(dataline : &str) -> i32
{
    let mut first_digit = '0';
    let mut last_digit = '0';
    for c in dataline.chars()
    {
        if c.is_digit(10)
        {
            if first_digit == '0'
            {
                first_digit = c
            }

            last_digit = c
        }
    }

    let mut output_string = String::from("");
    output_string.push(first_digit);
    output_string.push(last_digit);
    return output_string.parse().unwrap();
}

fn convert_option(option : &str) -> char
{
    match option
    {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => option.chars().nth(0).unwrap()
    }
}

fn get_real_first_and_last_digit(dataline : &str) -> i32
{
    let possible_options = 
    [
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    let mut first_digit_index : usize = dataline.len() + 1;
    let mut first_digit = '0';
    let mut last_digit_index : usize = 0;
    let mut last_digit = '0';

    for option in possible_options
    {
        let first_case = dataline.find(option);
        let last_case = dataline.rfind(option);
        if first_case != None
        {
            // We know there must be a value for both first and last case
            if first_case.unwrap() < first_digit_index
            {
                first_digit_index = first_case.unwrap();
                first_digit = convert_option(&option);
            }
            if last_case.unwrap() >= last_digit_index
            {
                last_digit_index = last_case.unwrap();
                last_digit = convert_option(&option);
            }
        }
    }

    let mut output_string = String::from("");
    output_string.push(first_digit);
    output_string.push(last_digit);
    return output_string.parse().unwrap();
}

pub fn main()
{
    let data = fs::read_to_string("inputs/day01.txt").expect("Failed to read file!");
    let mut value = 0;
    for dataline in data.split("\n")
    {
        value += get_first_and_last_digit(&dataline);
    }

    println!("Part 1: {value}");

    let mut new_value = 0;
    for dataline in data.split("\n")
    {
        new_value += get_real_first_and_last_digit(&dataline);
    }

    println!("Part 2: {new_value}");
}