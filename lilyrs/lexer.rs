use preprocessor;

pub fn lex(mut lily:preprocessor::LilyT) {
    let vcollection:Vec<&str> = lily.raw_buffer.split('\n').collect();
    let mut iterator:usize = 0;
    let mut aiterator:usize = 0;

    let n1:i32 = -1;

    lily.tokens.push(vec![]);
    lily.tokens[aiterator].push(String::new());
    for j in 0..vcollection.len() {
        if vcollection[j] == "" {
            continue;
        }

        let current:Vec<char> = vcollection[j].chars().collect();
        let mut instr:bool = false;

        for i in 0..vcollection[j].len() {
            let cchar:char = current[i];

            if cchar == '"' && instr == false {
                instr = true;
                lily.tokens[aiterator].push(String::new());
                lily.tokens[aiterator][iterator].push(cchar);
                continue;
            } else if cchar == '"' && instr == true {
                if n1.wrapping_add(i as i32) > -1 {
                    if current[i-1] == '\\' {
                        break;
                    }
                }
                instr = false;
                lily.tokens[aiterator][iterator].push(cchar);
                lily.tokens[aiterator].push(String::new());
                iterator += 1;
                continue;
            }

            if instr == false && (cchar == '(' || cchar == ')' || cchar == '{' || cchar == '}' || cchar == ',' || cchar == '.' || cchar == ' ') {   
                if cchar != ' ' {
                    lily.tokens[aiterator][iterator].push(cchar);
                    iterator += 1;
                    continue;
                }
                lily.tokens[aiterator].push(String::new());
                iterator += 1;
            } else {
                lily.tokens[aiterator][iterator].push(cchar);
            }
        } aiterator += 1;
        lily.tokens.push(vec![]);
        lily.tokens[aiterator].push(String::new());

        iterator = 0;
    }


}