use std::process;
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
        "remove".to_string(), "update".to_string()];
        if !allowed_queries.contains(&query){
            return Err(format!("{} is not a recognized command", query))
        }
        Ok(Args{init_command, query})
    }

}

pub fn run_app(items: &[String]){
    let _args: Args = Args::parse_args(items).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("First argument: {}, Second argument: {}", 
    _args.init_command, _args.query)
}