use clap::{arg, Parser};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct CmdLineArguments {
    /// Snippet name.
    #[arg(short, long, default_value_t = String::from("__NAME__"))]
    pub name: String,

    /// Snippet prefix.
    #[arg(short, long, default_value_t = String::from("__PREFIX__"))]
    pub prefix: String,

    /// Snippet description.
    #[arg(short, long, default_value_t = String::from("__DESCRIPTION__"))]
    pub description: String,

    /// If this option is specified,
    /// the snippet is defined as a file template snippet.
    #[arg(short, long)]
    pub template: bool,

    /// Add the code as a public module with the specified name.
    #[arg(short, long)]
    pub module_name: Option<String>,

    /// If this option is specified,
    /// only the body definition of the snippet is output.
    #[arg(short, long)]
    pub body_only: bool,
}
