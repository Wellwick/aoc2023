use std::fs;

struct Part
{
    x_loc : usize,
    y_loc : usize,
    val : char
}

fn gather_parts(data : &str) -> Vec<Part>
{
    let mut parts : Vec<Part> = Vec::new();
    for (y, dataline) in data.split("\n").enumerate()
    {
        for (x, character) in dataline.chars().enumerate()
        {
            if character != '.' && !character.is_digit(10)
            {
                parts.push(Part
                {
                    x_loc: x,
                    y_loc: y,
                    val: character
                });
            }
        }
    }

    parts
}

pub fn main()
{
    let data = fs::read_to_string("inputs/day02.txt").expect("Failed to read file!");
    let parts = gather_parts(&data);

    for part in parts
    {
        println!("{}", part.val);
    }

    //println!("Part 1: {value}");
}