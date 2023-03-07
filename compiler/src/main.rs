use std::io;
use std::io::Write;

mod eval;
mod lex;
mod lisp;
mod parse;
mod scan;

fn _run_source(source: &str, context: &mut eval::EvalContext) {
    let tokens = lex::lex(source).expect("Couldnt Lex!");
    let expressions = parse::parse(&tokens).expect("Couldnt Parse!");
    for expr in expressions {
        let res = eval::eval(&expr, context).expect("Couldnt Eval!");
        println!("{}", res);
    }
}

fn _prompt() -> std::io::Result<String> {
    let mut line = String::new();
    let stdin = io::stdin();
    print!(">> ");
    io::stdout().flush()?;
    stdin.read_line(&mut line)?;
    Ok(line)
}

fn main() -> std::io::Result<()> {
    println!("==== Welcome To Lisp! ====");
    let mut context = eval::EvalContext::new();

    loop {
        let line = _prompt()?;
        _run_source(&line, &mut context);
    }

    Ok(())
}
