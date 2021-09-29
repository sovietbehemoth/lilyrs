use std::env;

fn main() {
    let args:Vec<String> = env::args().collect();
    //println!("{}", args[2].as_str());
    let _err = match args[1].as_str() {
        "BHS0001" => {
            println!("Within the arguments supplied to the interpreter executable, a file was not supplied. Provide a file in the following syntax; ./lily <file_path>.");
        }

        "BHS0002" => {
            println!("The file supplied has an extension that is not '.bhs'. The '.bhs' extension is the only valid extension.");
        }
        
        _ => {
            println!("Error {:?} not found.", args[1]);
        }
    };
}