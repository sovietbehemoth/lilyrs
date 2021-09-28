mod preprocessor;
mod lexer;


fn main() {
    
    let mut lily = preprocessor::LilyT {
        raw_buffer: String::new(),
        file_name: String::new(),
        tokens: vec![],
    };
    
    lily.file_name = preprocessor::flag_check();
    lily.raw_buffer = preprocessor::file_read(&lily.file_name);

    lexer::lex(lily);
}