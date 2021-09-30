mod preprocessor;
mod lexer;
mod parser;
mod stack;

fn main() {
    let mut lily = preprocessor::LilyT {
        raw_buffer: String::new(),
        file_name: String::new(),
        tokens: vec![],
        context: preprocessor::Context {
            line: 0,
            st_offset: 0,
            st_iteration: 0,
        },
        stack: stack::Stack {
            str_stack: vec![],
            int_stack: vec![],
        }
    };

    lily.file_name = preprocessor::flag_check();
    lily.raw_buffer = preprocessor::file_read(&lily.file_name);

    lily = lexer::lex(lily);
    parser::parser_init(lily);
}