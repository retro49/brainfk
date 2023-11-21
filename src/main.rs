extern crate brainfk;
use brainfk::bf::prog::Program;
use brainfk::bf::AppResult;
use brainfk::bf_help;

use clap::Arg;


fn start_repl() -> AppResult {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.to_lowercase().as_str().trim(){
        "exit" => {
            std::process::exit(0);
        },
        _ => {
            let mut prog = Program::new(input);
            match prog.execute(){
                Err(e) => {
                    return Err(e);
                },
                _ => {}
            }
        }
    };
    start_repl().unwrap();
    Ok(())
}


fn main() -> AppResult {
    if std::env::args().len() < 2 {
        start_repl()?;
    }

    // build the commands
    let cmd = clap::Command::new("brainfk")
        .author("retro")
        .about("simple brain-f*ck interpreter")
        .version("1.0.0.0")
        .arg(
            Arg::new("core")
                .short('c')
                .long("core")
                .required(false)
                .help("core dump file")
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .help("byte format style for the core dump")
                .required(false)
                .value_parser(clap::value_parser!(u8))
                .default_value("8")
        )
        .arg(
            Arg::new("file")
                .required(true)
                .last(true)
                .help("input file")
        )
        .arg(
            Arg::new("help")
                .long("help")
                .short('h')
                .help(bf_help::show_full_help())
        )
        .get_matches();
    
    /*
    let input = r#"
    >++++++++[<+++++++++>-]<.>++++[<+++++++>-]
    <+.+++++++..+++.>>++++++[<+++++++>-]<+
    +.------------.>++++++[<+++++++++>-]<+.
    <.+++.------.--------.>>>++++[<++++++++>-]<+.
    "#.to_string();
    let mut program = Program::new("+++++[>++++++++++<-]>+++++++++++++++.".to_string());
    //let mut program = Program::new(input);
    program.execute().unwrap();
    */
    Ok(())
}
