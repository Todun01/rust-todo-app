use std::env;
use std::process;
fn main() {
    let _args: Vec<String> = env::args().collect();
    run_app(&_args);
}

struct Args{
    init_command: String,
    query: String,
    item: Option<String>
}

impl Args {
    pub fn parse_args(_args: &[String]) -> Result <Args, &str>{
        if _args.len() < 3{
            return Err("You need to enter more arguments")
        }
        if _args.len() == 3{
            let init_command = _args[1].clone();
            let query = _args[2].clone();
            let item = None;
            return Ok(Args{init_command, query, item});
        }
        let init_command = _args[1].clone();
        let query = _args[2].clone();
        let item = Some(_args[3].clone());
        Ok(Args{init_command, query, item})

        
    }
}

pub fn run_app(items: &[String]){
    let _args: Args = Args::parse_args(items).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("First argument: {}, Second argument: {}, third argument: {:?}", _args.init_command, _args.query, _args.item)
}