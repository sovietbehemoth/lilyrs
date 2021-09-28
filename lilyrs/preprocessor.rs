use std::io::Read;
use std::fs::File;
use std::process;


pub struct LilyT {
    //Source file contents.
    pub raw_buffer:String,
    //Name of source file.
    pub file_name:String,
    //Token set created by lexical analysis.
    pub tokens:Vec< Vec<String> >,
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

/*
I just want to play the sport I love, I don't really want to write a random rust interpreter right now. It's not
fair that everyone else gets to play and I don't. I know my grades were bad last year but I have been working very
hard to make up for it this year, I don't want to be punished for something that i've been working hard to make up
for. Why must I be excluded for struggling in a year that is so abnormal with a severe learning condition that 
impairs my ability to focus? Why must every time I finally start having a period of content and happiness must it
get interrupted with something impairing that? I am a human, I have wants and desires, I don't want to have this
happen, it seems like this is not fair. My ADHD is not something that is easy to manage on the daily. I'm sorry that I neglected my studies and constantly
lost focus. I'm sorry that I didn't keep up with my assignments accordingly. I'm sorry that I didn't meet the 
expectations, I just had a very hard time adjusting to such an abnormal learning environment. Please let me play my
sport, I have been working so hard for so long. I've been training every day, I just want to run. Please let me run,
I know that I have made mistakes but I make them not in mal intent. I am not a perfect person, I am far from it. I've 
made many mistakes in the time that i've been alive. I just want to have some fun for a while. I'm sorry for the 
mistakes that i've made, I take full responsibility for all troubles and conflict that I have caused. I just want to
have fun and have a break from having to deal with being left out and being an outsider for once. Cross country was 
my sense of inclusion, my chance at belonging to something. There has not been very many times in 
my life where i've truly felt like I belonged. This is one of the few times. I feel like I belong to this group and
that I am one of them. I feel not like an intruder or outsider, but like a teammate and friend. Being excluded from 
the team may seem like a minor inconvenience, but it really is something that impacts me greatly.
I don't have an easy time making friends and I was just getting established in that group of people, not to mention
that I worked for so long to get into the conditioning that I am in. 

Please, please don't take this away from me. Please, I beg you. I just want to belong, I just want to be a part of 
something, I want to have something to do other than school. Please let me have my sports, it's the only group in 
which I truly belong.

You only appreciate something after you have it taken from you.
*/