use preprocessor;

fn parser_visit(lily:&preprocessor::LilyT) {
    for i in 0..lily.tokens[lily.context.line as usize].len() {
        let statement = &lily.tokens[lily.context.line as usize][i];
        match statement.as_str() {
            "const" | "let" => {
                println!("d");
            },
            _ => (),
        }
    }
}

pub fn parser_init(mut lily:preprocessor::LilyT) {
    for i in 0..lily.tokens.len() {
        lily.context.line = i as i64;
        parser_visit(&lily);
    }
}