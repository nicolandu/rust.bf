use anyhow::Result;
use clap::{Parser, Subcommand};
use todo::{Entry, Todo};

#[derive(Parser, Debug)]
#[command(name = "todo")]
#[command(about = "A simple command-line todo list.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Add one or more entries (space-separated, use quotes to include spaces).
    #[command(visible_alias = "a", arg_required_else_help = true)]
    Add {
        #[arg(num_args = 1..)]
        labels: Vec<String>,
    },

    /// Remove one or more entries (space-separated indices).
    #[command(
        visible_alias = "r",
        visible_alias = "rm",
        arg_required_else_help = true
    )]
    Remove {
        /// The entry's indices (space-separated indices).
        #[arg(num_args = 1.., value_name = "INDICES")]
        indices: Vec<usize>,
    },

    /// Mark one or more entries as done (space-separated indices).
    #[command(visible_alias = "d", arg_required_else_help = true)]
    Done {
        /// The entry's indices
        #[arg(num_args = 1.., value_name = "INDICES")]
        indices: Vec<usize>,
    },

    /// Mark one or more entries as not done (space-separated indices).
    #[command(visible_alias = "u", arg_required_else_help = true)]
    Undo {
        /// The entry's indices
        #[arg(num_args = 1.., value_name = "INDICES")]
        indices: Vec<usize>,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut todo = Todo::open()?;
    if let Some(command) = args.command {
        match command {
            Commands::Add { labels } => { todo.add(labels.iter().map(|l| Entry { label: l.chars().filter_map(|c| if !c.is_control() {Some(c)} else if c.is_whitespace() {Some(' ')} else {None}).collect(), done: false, }));
            }
            Commands::Remove { indices } => match todo.remove(&indices) {
                Ok(_) => (),
                Err(e) => println!("{e}"),
            },
            Commands::Done { indices } => match todo.set_done(&indices, true) {
                Ok(_) => (),
                Err(e) => println!("{e}"),
            },
            Commands::Undo { indices } => match todo.set_done(&indices, false) {
                Ok(_) => (),
                Err(e) => println!("{e}"),
            },
        }

        todo.save()?;
    }
    todo.display();
    Ok(())
}
