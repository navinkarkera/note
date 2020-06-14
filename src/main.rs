use colored::*;
use dialoguer::Editor;
use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

mod data;

#[derive(Debug, StructOpt)]
#[structopt(name = "Note", about = "Take short notes")]
struct Cli {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(help = "List n last notes, default n = 5")]
    List { number: Option<usize> },
    #[structopt(help = "Search notes")]
    Search { text: String },
}

fn run() -> Result<(), Box<dyn Error>> {
    let opt = Cli::from_args();
    match opt.cmd {
        Some(command) => match command {
            Command::List { number } => match number {
                Some(num) => {
                    let note_list = read_note_list()?;
                    println!("{}", list_notes(&note_list, num))
                }
                None => {
                    let note_list = read_note_list()?;
                    println!("{}", list_notes(&note_list, 5))
                }
            },
            Command::Search { text } => {
                let note_list = read_note_list()?;
                let notes: Vec<_> = note_list
                    .into_iter()
                    .filter(|n| n.contains(&text))
                    .collect();
                if notes.len() > 0 {
                    println!("{}", list_notes(&notes, notes.len()));
                } else {
                    println!("No notes found!");
                }
            }
        },
        None => {
            let mut editor = &mut Editor::new();
            if let Ok(executable_path) = env::var("NOTES_EDITOR") {
                editor = editor.executable(executable_path)
            }
            if let Some(note_text) = editor
                .require_save(true)
                .extension(".md")
                .edit("")
                .expect("Please set a default editor")
            {
                let note = data::Note::create(note_directory(), note_text);
                println!("{}", note);
                note.save().expect("Failed to save note!!");
            } else {
                println!("Abort saving note!");
            }
        }
    }

    Ok(())
}

fn note_directory() -> PathBuf {
    let note_dir_path = env::var("NOTES_DIR").expect("Please set NOTES_DIR env var!!");
    PathBuf::from(note_dir_path)
}

fn read_note_list() -> Result<Vec<String>, std::io::Error> {
    let note_dir = note_directory();
    let mut note_list = Vec::new();
    for note in note_dir.read_dir()? {
        let note = note?;
        let note_path = note.path();
        if !note_path.is_dir() {
            note_list.push(read_to_string(note.path())?);
        }
    }
    Ok(note_list)
}

fn list_notes(note_list: &Vec<String>, num: usize) -> String {
    note_list
        .iter()
        .rev()
        .take(num)
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(&format!(
            "\n{}\n\n",
            "===================================".green()
        ))
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
