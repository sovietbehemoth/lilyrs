use std::io::Read;
use std::fs::File;
use std::process;

use stack;


pub struct Context {
  pub line:i64,
  pub st_iteration:i64,
  pub st_offset:i64,
}

pub struct LilyT {
    //Source file contents.
    pub raw_buffer:String,
    //Name of source file.
    pub file_name:String,
    //Token set created by lexical analysis.
    pub tokens:Vec< Vec<String> >,

    pub context:Context,
    pub stack:stack::Stack,
}

fn preprocessor_error(msg:&str) -> ! {
    eprintln!("Error[PreprocessorError]: {}", msg);
    process::exit(0);
}

//Read file.
pub fn file_read(fpath:&str) -> String {
    let mut file = match File::open(&fpath) {
        Err(_) => preprocessor_error( format!("File '{}' not found", fpath).as_str() ),
        Ok(file) => file,
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Err(_) => preprocessor_error( format!("File '{}' could not be read.", fpath).as_str() ),
        Ok(file) => file,
    };

    buffer
} 

//Check command line options. 
pub fn flag_check() -> String {
    let args:Vec<String> = std::env::args().collect();
    
    if args.len() == 1 {
        preprocessor_error("Expected file name or other option.");
    }

    for i in 1..args.len() {
      let arg:&String = &args[i];
      if arg.ends_with(".lily") {
          return String::from(arg)
      } else {
        match arg.as_str() {
          "-v" => {
            println!("Lily V1");
            process::exit(0);
          },
          def => {
            preprocessor_error( &format!("Invalid option '{}'.", def) );
          }
        }
      }
    }

    /*Compiler gets angry when this is omitted because it expects String to return. String
    is the only type that will return. This code will never get executed as all other portions
    of the loop end in breakpoints.*/
    String::from("")
  }

