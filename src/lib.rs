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

pub fn add_item< 'a>(item: & 'a str, file_name: & 'a str) -> Result<(), Box<dyn Error>>{
    let filetext = fs::read_to_string(file_name)?;
    let mut count = 0;
    for line in filetext.lines(){
        count += 1;
    }
    if !Path::new(file_name).exists(){
        println!("That file doesn't exist");
        return Ok(())
    }

    let mut file = fs::OpenOptions::new().append(true).open(file_name)?;
    writeln!(file, "{}• {}", count + 1, item)?;
    println!("Item added successfully!");

    Ok(())
}
pub fn remove_item< 'a>(line_no: i64, file_name: & 'a str) -> Result<(), Box<dyn Error>>{
    let filetext = fs::read_to_string(file_name)?;
    let file = fs::OpenOptions::new().read(true).open(file_name)?;
    let mut _found: bool = false;
    let mut count: i64 = 1;
    for _line in filetext.lines(){
        if count == line_no{
            _found = true;
            let reader = io::BufReader::new(file);
            let lines: Vec<String> = reader
            .lines()
            .filter_map(Result::ok)
            .filter(|_line| true)
            .collect();
            let mut file = fs::OpenOptions::new().write(true).truncate(true).open(file_name)?;
            for line in lines{
                writeln!(file, "{}", line);
            }
            println!("Item removed successfully!");
            return Ok(());
        }
        count += 1
    }
    if count < line_no{
        println!("Oops. Nothing to remove here");
    }
    
    Ok(())
}
pub fn update_item(new_item: &str, old_item_no: &str, filename: &str) -> Result<(), Box <dyn Error>>{
    let filetext = fs::read_to_string(filename)?;
    let mut file = fs::OpenOptions::new().write(true).truncate(true).open(filename)?;
    let mut lines: Vec<String> = vec![];
    let mut found = false;
    let mut count = 1;
    for line in filetext.lines(){
        if line.split('•').nth(0) == Some(old_item_no){
            found = true;
            lines.push(format!("{}• {}", count, new_item.to_string()));
        } else{
            lines.push(line.to_string())
        }
        count += 1;
    }
    if found{
        for line in lines{
            writeln!(file, "{}",line)?;
        }
        println!("Item updated successfully!")
    } else{
        println!("There is no item at number {}", old_item_no)
    }
    Ok(())
}
pub fn count_line(filename: &str) -> Result <i64 , Box<dyn Error>>{
    let filetext = fs::read_to_string(filename)?;
    let mut count: i64 = 0;
    for line in filetext.lines(){
        count += 1
    };
    Ok(count)
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
        let result = add_item(&item.trim_end(), &filename.trim_end());
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
    if _args.query == "update"{
        let mut filename = String::new();
        let mut old_item_no = String::new();
        let mut new_item = String::new();
        println!("Please enter the list you want to update:");
        io::stdin().read_line(&mut filename).expect("Failed to read file name");
        println!("These are the items on your list: ");
        if let Err(e) = show_list(&filename.trim_end()) {
            eprintln!("Application error: {}", e );
            process::exit(1)
        };
        println!("Please enter the number of the item you want to update:");
        io::stdin().read_line(&mut old_item_no).expect("Failed to read old item number");
        println!("Please enter the new item:");
        io::stdin().read_line(&mut new_item).expect("Failed to read new item");
        if let Err(e) = update_item(&new_item.trim_end(), &old_item_no.trim_end(), &filename.trim_end()){
            eprintln!("Application Error: {}", e);
            process::exit(1)
        }

    }
    if _args.query == "remove"{
        let mut filename = String::new();
        let mut line_no = String::new();
        println!("Please enter the list you want to remove from:");
        io::stdin().read_line(& mut filename).expect("Failed to read file name");
        println!("Please enter the item you want to remove from {}:", filename);
        io::stdin().read_line(& mut line_no).expect("Failed to read item");
        let line_num: i64 = line_no.parse().unwrap();
        if let Err(e) = remove_item(line_num, &filename.trim_end()) {
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
    let item = "do something";
    if let Err(e) = add_item(item.trim_end(), "/home/oluwatodunni/Documents/rust-todo-app/another one.txt") {
        eprintln!("APplication error: {}", e);
        process::exit(1)
    }
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
    if let Err(e) = remove_item(3, filename){
        eprintln!("Failed to remove item");
        process::exit(1)
    }
}

#[test]
fn update_test(){
    let filename = "another one.txt";
    let old_item_no = "2";
    let new_item = "do something tomorrow";
    if let Err(e) = update_item(new_item, old_item_no, filename){
        eprintln!("Failed to update item");
        process::exit(1)
    }
}