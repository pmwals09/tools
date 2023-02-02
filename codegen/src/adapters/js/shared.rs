use clap::{Args, Subcommand, ValueEnum};

#[derive(Args, Debug)]
pub struct JS {
    /// Generator to use
    #[command(subcommand)]
    pub generator: JSGenerator,
}

#[derive(Debug, Subcommand)]
pub enum JSGenerator {
    Class(Class),
    ReactComponent(ReactComponent),
}

#[derive(Args, Debug)]
pub struct Class {
    /// Filename and generated item name
    #[arg(short, long)]
    pub name: String,
    /// Any additional properties relevant to the generated item
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    pub properties: Vec<String>,
}

#[derive(Args, Debug)]
pub struct ReactComponent {
    /// Filename and generated item name
    #[arg(short, long)]
    pub name: String,
    /// What support files to include
    #[arg(short, long, num_args = 1.., value_delimiter = ' ')]
    pub file_include: Option<Vec<ReactSupport>>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum ReactSupport {
    Style,
    Config,
    Service,
    Test
}

pub fn capitalize(s: &str) -> String {
    let mut new_s = String::new();
    let mut cs = s.chars();
    let first = cs.next().expect("Should have at least one character");
    new_s.push(first.to_ascii_uppercase());
    while let Some(c) = cs.next() {
        new_s.push(c);
    }
    new_s
}
