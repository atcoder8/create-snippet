use crate::code_to_snippet::{code_to_snippet_body, create_snippet};
use clap::Parser;

mod cmdline_args;
mod code_to_snippet;

fn main() {
    let args = cmdline_args::CmdLineArguments::parse();

    let code: Vec<String> = std::io::stdin()
        .lines()
        .map(|line| {
            if let Ok(line) = line {
                line
            } else {
                panic!("Failed to read input from stdin.");
            }
        })
        .collect();

    if args.body_only {
        print!("{}", code_to_snippet_body(&code));
    } else {
        print!(
            "{}",
            create_snippet(
                &code,
                &args.name,
                &args.prefix,
                &args.description,
                args.template,
                args.module_name.as_deref(),
            )
        );
    }
}
