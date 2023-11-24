extern crate brainfk;
use brainfk::bf::prog::Program;
use brainfk::bf::AppResult;
use brainfk::bf_runner::Base;
use brainfk::bf_runner::Runner;

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
            Arg::new("base")
                .short('b')
                .long("base")
                .help("byte format style for the core dump")
                .required(false)
                .value_parser(clap::value_parser!(u8))
                .default_value("10")
        )
        .arg(
            Arg::new("file")
                .required(true)
                .last(true)
                .help("input file")
        )
        .get_matches();

    let file = match cmd.get_one::<String>("file"){
        Some(f) => {f},
        None => panic!("no input file provided to run"),
    };

    let core = cmd.get_one::<String>("core");
    let base = cmd.get_one::<u8>("base");
    if core != None {
        let core = core.unwrap();
        let base: Base = (*base.unwrap()).into();
        let mut runner = crate::brainfk::bf_runner::Runner::new(
            Some(base),
            Some(core.to_string()),
            file,
        );
        runner.execute()?;
    } else {
        Runner::new(None, None, file).execute()?;
    }

    Ok(())
}
