use anyhow::Result;
use serde_json::json;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use weather_parser::parse_input;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: run code using specific arguments\n");
        println!("cargo run <command> [options]\n");
        println!("Commands:");
        println!(
            "  parse <input_file> <output_file>  -Parse the input file and save to output file"
        );
        println!("  help                              -Show help information");
        println!("  credits                           -Show credits");
        return Ok(());
    }

    match args[1].as_str() {
        "parse" => {
            if args.len() != 4 {
                eprintln!("Error: 'parse' command requires an input file and an output file.");
                return Ok(());
            }
            let input_file_name = &args[2];
            let output_file_name = &args[3];

            let mut file = File::open(input_file_name)?;
            let mut input = String::new();
            file.read_to_string(&mut input)?;

            match parse_input(&input) {
                Ok(weather_report) => {
                    let weather_json = json!(weather_report);
                    let formatted_json = serde_json::to_string_pretty(&weather_json).unwrap();

                    let mut output_file = File::create(output_file_name)?;
                    output_file.write_all(formatted_json.as_bytes())?;

                    println!("JSON data has been written to {}", output_file_name);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        "help" => {
            println!("Usage: run code using specific arguments\n");
            println!("cargo run <command> [options]\n");
            println!("Commands:");
            println!(
                "  parse <input_file> <output_file>  -Parse the input file and save to output file"
            );
            println!("  help                              -Show help information");
            println!("  credits                           -Show credits");
            println!("\nAdditional explanations:\n");
            println!("To use this parser you must have data in format, displayed at input.txt");
            println!("General structure:");
            println!("Condition: *condition_type*; Temperature: *temperature*C; Humidity: *humidity*%; Wind: *direction* *speed*km/h; Precipitation: *precipitation_type* *amount*mm; Visibility: *distance*km; Cloud Cover: *percentage*%; Pressure: *pressure*hPa; UV Index: *uv_index*; Air Quality: *air_quality_level*; Sunrise: *time*; Sunset: *time*; Cloud Types: *cloud_type*, *cloud_type*, ...");
            println!("Write all of it in one line.");
            println!("You will get JSON on the out.");
        }
        "credits" => {
            println!("Weather Parser");
            println!("Developed by Andrii Kviatkovskyi");
            println!("Student of NaUKMA SE-3");
        }
        _ => {
            eprintln!("Unknown command");
            println!("Run 'cargo run help' to see available commands.");
        }
    }
    Ok(())
}
