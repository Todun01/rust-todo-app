use std::process;
use std::fs;
use std::io;
use std::io::Write;
use std::io::{Error, ErrorKind};
use std::path::Path;
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

pub fn show_list(filename: &str) -> Result<String, std::io::Error>{
    let _filetext = fs::read_to_string(filename)?;
    if _filetext.is_empty() {
        let error_message = Error::new(ErrorKind::Other, "You haven't added any items yet");
        return Err(error_message)
    }
    Ok(_filetext)
}

pub fn add_item< 'a>(item: & 'a str, file_name: & 'a str) -> Result<& 'a str, std::io::Error>{
    if !Path::new(file_name).exists(){
        let error_message =  Error::new(ErrorKind::Other, "That file doesn't exist");
        return Err(error_message)
    }

    let mut file = fs::OpenOptions::new().append(true).open(file_name)?;
    writeln!(file, "{}", item)?;

    Ok("Item added successfully!")
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
    let result = add_item(item, "/home/oluwatodunni/Desktop/testfile.txt");
    println!("{:?}", result)
}