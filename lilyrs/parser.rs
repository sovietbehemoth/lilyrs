use preprocessor;

use std::io::Write;
use std::io;
//helloworld
pub fn error(msg:&str, errtype:i64, errcode:&String, highlight:&String, lily:&preprocessor::LilyT) -> ! {
    let indexf:Vec<_> = errcode.as_str().match_indices(highlight.as_str()).collect();
    let index:usize = indexf[0].0;

    let mut sqg = String::new();
    for i in index..errcode.len() {
        if (errcode.len() - i) - 2 == index {
            break;
        }
        sqg.push('~');
    } 
    eprint!("Error[LILY{}]: {}\n\t--> {}:{}:{}\n|\n|\t{}|", errtype, msg, lily.file_name, lily.context.line+1, index, errcode);
    eprintln!("\t{}", sqg);
    std::process::exit(1);
}

#[feature(split_inclusive)] 
fn statement_visit(mut lily:preprocessor::LilyT) -> preprocessor::LilyT {
    let st = &lily.tokens[lily.context.line as usize];
    let mut cl:i8 = -1;


    if st[0] == "const" {
        cl = 0;
    } else {
        cl = 1;
    }

    if st.len() == 1 {
        let cr:Vec<&str> = lily.raw_buffer.split_inclusive('\n').collect();
        error(
            &format!("Expected statement after '{}'.", st[0]).as_str(),
            0001, &String::from(cr[lily.context.line as usize]), &st[0], &lily
        );
    }

    for i in 1..st.len() {
    }
    lily
}

fn parser_visit(mut lily:preprocessor::LilyT) -> preprocessor::LilyT {
    for i in 0..lily.tokens[lily.context.line as usize].len() {
        let statement = &lily.tokens[lily.context.line as usize][i];
        match statement.as_str() {
            "const" | "let" => {
                lily = statement_visit(lily);
            },
            _ => (),
            "" => (),
        }
    }
    lily
}

pub fn parser_init(mut lily:preprocessor::LilyT) {
    for i in 0..lily.tokens.len() {
        lily.context.line = i as i64;
        lily = parser_visit(lily);
    }
}