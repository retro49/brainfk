use brainfk::bf::prog::Program;

fn start_repl(){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.to_lowercase().as_str().trim(){
        "exit" => {
            std::process::exit(0);
        },
        _ => {
            let mut prog = Program::new(input);
            match prog.execute(){
                Ok(_) => {
                    start_repl();
                },
                Err(e) => {
                    // please try adding some logging
                    println!("error: {}", e);
                },
            }
        }
    }
    start_repl();
}

fn main() {
    let args = std::env::args().skip(1);
    if args.len() < 1 {
        println!("*** BRAIN F*CK REPL ***");
        start_repl();
    }

    // handle file

    let input = r#"
    >++++++++[<+++++++++>-]<.>++++[<+++++++>-]
    <+.+++++++..+++.>>++++++[<+++++++>-]<+
    +.------------.>++++++[<+++++++++>-]<+.
    <.+++.------.--------.>>>++++[<++++++++>-]<+.
    "#.to_string();
    let mut program = Program::new(input);
    program.execute().ok();
}
