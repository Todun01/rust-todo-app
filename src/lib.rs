use std::process;
use std::fs;
use std::io;
use std::io::{BufReader, BufRead, Write};
use std::path::Path;
use std::error::Error;
pub struct Args{
    init_command: String,
    query: String
}

impl Args {
    pub fn parse_args(_args: &[String]) -> Result <Args, String>{
        if _args.len() < 3{
            return Err(format!("You need to enter more arguments"))
        }
        if _args.len() > 3{
            return Err(format!("You entered too many arguments"))
        }
        let init_command = _args[1].clone();
        let query = _args[2].clone();
        if init_command != "todoapp"{
            return Err(format!("{} is not a recognized command", init_command))
        }
        let allowed_queries = vec!["add".to_string(), "show".to_string(), 
        "remove".to_string(), "update".to_string(), "new_list".to_string()];
        if !allowed_queries.contains(&query){
            return Err(format!("{} is not a recognized command", query))
        }
        Ok(Args{init_command, query})
    }

}
pub fn new_list(name: &str) -> Result<&str, &str>{
    fs::File::create(&format!("{}.txt",name));
    if Path::new(name).exists(){
        return Err("Error creating file")
    }
    Ok("File created successfully!")
}

pub fn show_list(filename: &str) -> Result<(), Box<dyn Error>>{
    let _filetext = fs::read_to_string(filename)?;
    println!("{}",_filetext);
    Ok(())
}

pub fn add_item< 'a>(item: & 'a str, file_name: & 'a str) -> Result<& 'a str, std::io::Error>{
    if !Path::new(file_name).exists(){
        println!("{:?}", Path::new(file_name));
        let error_message =  io::Error::new(io::ErrorKind::Other, "That file doesn't exist");
        return Err(error_message)
    }

    let mut file = fs::OpenOptions::new().append(true).open(file_name)?;
    writeln!(file, "• {}", item)?;

    Ok("Item added successfully!")
}
pub fn remove_item< 'a>(item: & 'a str, file_name: & 'a str) -> Result<(), Box<dyn Error>>{
    let filetext = fs::read_to_string(file_name)?;
    let file = fs::OpenOptions::new().read(true).open(file_name)?;
    let mut _found: bool = false;
    for line in filetext.lines(){
        if line.contains(item){
            _found = true;
            let reader = io::BufReader::new(file);
            let lines: Vec<String> = reader
            .lines()
            .filter_map(Result::ok)
            .filter(|line| !line.contains(item))
            .collect();
            let mut file = fs::OpenOptions::new().write(true).truncate(true).open(file_name)?;
            for line in lines{
                writeln!(file, "{}", line);
            }
            println!("Item removed successfully!");
            return Ok(());
        }
    }
    if !_found{
        println!("Item not found");
    }
    
    Ok(())
}

pub fn run_app(items: &[String]){
    let _args: Args = Args::parse_args(items).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if _args.query == "new_list"{
        let mut filename = String::new();
        println!("Please enter the name of the file:");
        io::stdin().read_line(& mut filename).expect("Failed to read input");
        let result = new_list(&filename);
        println!("{:?}", result)
    }
    if _args.query == "add"{
        let mut filename = String::new();
        let mut item = String::new();
        println!("Please enter the name of your list:");
        io::stdin().read_line(& mut filename).expect("Failed to read file");
        println!("Please enter the item you want to add:");
        io::stdin().read_line(& mut item).expect("Failed to read input");
        let result = add_item(&item, &filename.trim_end());
        println!("{:?}", result)

    }
    if _args.query == "show" {
        let mut filename = String::new();
        println!("PLease enter the list you want to display:");
        io::stdin().read_line(& mut filename).expect("Failed to read file");
        if let Err(e) = show_list(&filename.trim_end()) {
            eprintln!("Application error: {}", e );
            process::exit(1)
            
        };
    }
    if _args.query == "remove"{
        let mut filename = String::new();
        let mut item = String::new();
        println!("Please enter the list you want to remove from:");
        io::stdin().read_line(& mut filename).expect("Failed to read file name");
        println!("Please enter the item you want to remove from {}:", filename);
        io::stdin().read_line(& mut item).expect("Failed to read item");
        if let Err(e) = remove_item(&item.trim_end(), &filename.trim_end()) {
            eprintln!("Application error: {}", e);
            process::exit(1)
        }
    }
    // println!("First argument: {}, Second argument: {}", 
    // _args.init_command, _args.query)
}

#[cfg(test)]

mod tests{
    use super::*;
}

#[test]
fn file_testing(){
    let file_name = "/home/oluwatodunni/Desktop/testfile.txt";
    let result = show_list(file_name);
    println!("Output: {:?}", result)
}

#[test]
fn file_creation(){
    let file_name = "testing tings";
    let result = new_list(file_name);
    println!("{:?}", result)
}

#[test]
fn file_append(){
    let item = "remember to call mom";
    let result = add_item(item, "/home/oluwatodunni/Documents/rust-todo-app/another one.txt");
    println!("{:?}", result)
}

#[test]

fn show_file(){
    let filename = "another one.txt";
    if let Err(e) = show_list(filename) {
        eprintln!("Application error: {}", e );
        process::exit(1)
        
    };
}

#[test]
fn remove_test(){
    let filename = "another one.txt";
    let item = "call mom";
    if let Err(e) = remove_item(item, filename){
        eprintln!("Failed to remove item");
        process::exit(1)
    }
}