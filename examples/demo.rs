use cliproc::{cli, proc};
use cliproc::{Cli, Command, ExitCode, Help, Memory, Optional};
use std::env;

// 1. Define the struct and the data required to perform its task
struct Demo {
    name: String,
    count: Option<u8>,
}

// 2. Implement the `Command` trait to allow a struct to function as a command
impl Command for Demo {
    // 2a. Map the command-line data to the struct's data
    fn interpret(cli: &mut Cli<Memory>) -> cli::Result<Self> {
        cli.check_help(Help::default().text(HELP))?;
        Ok(Demo {
            name: cli.require_option(Optional::new("name").switch('n'))?,
            count: cli.check_option(Optional::new("count").switch('c'))?,
        })
    }

    // 2b. Process the struct's data to perform its task
    fn execute(self) -> proc::Result {
        for _ in 0..self.count.unwrap_or(1) {
            println!("Hello {}!", self.name);
        }
        Ok(())
    }
}

// 3. Parse the command-line arguments and run the command
fn main() -> ExitCode {
    Cli::default().parse(env::args()).go::<Demo>()
}

const HELP: &str = "\
A fast, low-level, and configurable command-line processor.

Usage:
    demo [options] --name <name>
    
Options:
    --name, -n <name>       Name of the person to greet              
    --count, -c <count>     Number of times to greet (default: 1)
    --help, -h              Print this help information and exit
";
