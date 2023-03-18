use std::env;
use std::fs;
use std::io;
use std::io::Write;
use lisp;

mod eval;
mod lex;
mod parse;
mod scan;

fn _run_source(source: &str, context: &mut eval::frame::EvalContext, verbose: bool) {
    let tokens = lex::lex(source).expect("Couldnt Lex!");
    for expr in parse::parse(&tokens) {
        let res = eval::eval(&expr.expect("Could Not Parse!"), context).expect("Couldnt Eval!");
        if verbose {
            println!("{}", res);
        }
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

fn repl() -> std::io::Result<()> {
    println!("==== Welcome To Lisp! ====");
    let frame = eval::frame::EvalFrame::empty();
    let mut context = eval::frame::EvalContext::new(frame);

    loop {
        let line = _prompt()?;
        _run_source(&line, &mut context, true);
    }
}

fn execute_file(path: &str) -> std::io::Result<()> {
    let source = fs::read_to_string(path)?;
    let frame = eval::frame::EvalFrame::empty();
    let mut context = eval::frame::EvalContext::new(frame);
    _run_source(&source, &mut context, false);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match &args[..] {
        [.., _, path] => execute_file(path),
        _ => repl()
    }
}
