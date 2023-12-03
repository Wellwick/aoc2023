use std::fs;

struct CubeSet
{
    red : u32,
    green : u32,
    blue : u32
}

fn create_cube_set(dataset : &str) -> CubeSet
{
    let data = dataset.split(",");
    let mut cube_set = CubeSet
    {
        red:0,
        green:0,
        blue:0
    };
    for val in data
    {
        let number = val.trim().split_whitespace().next().expect("Does not seem to be set").parse().unwrap();
        if val.contains("red")
        {
            cube_set.red = number;
        }
        if val.contains("green")
        {
            cube_set.green = number;
        }
        if val.contains("blue")
        {
            cube_set.blue = number;
        }
    }

    return cube_set;
}

fn is_game_possible(dataline : &str, max_red : u32, max_green : u32, max_blue : u32) -> bool
{
    let datasets = dataline.split(":").last().expect("Does not seem to be game").split(";");
    for dataset in datasets
    {
        let cube_set = create_cube_set(dataset);
        if cube_set.red > max_red || cube_set.green > max_green || cube_set.blue > max_blue
        {
            return false;
        }
    }

    return true;
}

fn get_game_number(dataline : &str) -> u32
{
    return dataline.split(":").next().expect("Does not seem to be game").split_whitespace().last().expect("Still not a game").parse().unwrap();
}

fn get_game_power(dataline : &str) -> u32
{
    
    let datasets = dataline.split(":").last().expect("Does not seem to be game").split(";");
    let mut general_cubeset = CubeSet
    {
        red: 0,
        green: 0,
        blue: 0
    };

    for dataset in datasets
    {
        let cube_set = create_cube_set(dataset);
        if general_cubeset.red < cube_set.red
        {
            general_cubeset.red = cube_set.red;
        }
        if general_cubeset.green < cube_set.green
        {
            general_cubeset.green = cube_set.green;
        }
        if general_cubeset.blue < cube_set.blue
        {
            general_cubeset.blue = cube_set.blue;
        }
    }

    return general_cubeset.red * general_cubeset.green * general_cubeset.blue;
}

pub fn main()
{
    let data = fs::read_to_string("inputs/day02.txt").expect("Failed to read file!");
    let mut value = 0;
    for dataline in data.split("\n")
    {
        if is_game_possible(&dataline, 12, 13, 14)
        {
            value += get_game_number(&dataline);
        }
    }

    println!("Part 1: {value}");

    let mut new_value = 0;
    for dataline in data.split("\n")
    {
        new_value += get_game_power(&dataline);
    }

    println!("Part 2: {new_value}");
}