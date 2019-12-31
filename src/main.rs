use colored::*;
use dialoguer::{Confirmation, Editor};
use std::env;
use std::error::Error;
use std::fs::{read_to_string, File};
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
    #[structopt(help = "Clear all notes")]
    Clear,
    #[structopt(help = "Set local notes for this folder")]
    Local,
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
            Command::Clear => {
                if Confirmation::new()
                    .with_text("Do you want clear all notes?")
                    .default(false)
                    .interact()?
                {
                    clear_notes();
                    println!("Cleared!!")
                } else {
                    println!("Abort!");
                }
            }
            Command::Local => {
                set_local();
            }
        },
        None => {
            if let Some(note_text) = Editor::new().edit("").expect("Please set a default editor") {
                let note = data::Note::create(note_text);
                println!("{}", note);
                let mut note_list = read_note_list()?;
                note_list.push(note);
                write_notes(note_list);
            } else {
                println!("Abort saving note!");
            }
        }
    }

    Ok(())
}

fn note_file() -> PathBuf {
    let mut note_file_path = PathBuf::from(".notes.json");
    if !note_file_path.exists() {
        note_file_path = match env::var("NOTES_FILE") {
            Ok(note_file) => PathBuf::from(note_file),
            Err(_) => PathBuf::from(&env::var("HOME").unwrap()).join(".notes.json"),
        };
    }
    note_file_path
}

fn read_note_list() -> Result<Vec<data::Note>, std::io::Error> {
    let note_file = note_file();
    if note_file.exists() {
        let read_data = read_to_string(note_file)?;
        if read_data.is_empty() {
            return Ok(Vec::new());
        }
        let deserialized: Vec<data::Note> = serde_json::from_str(&read_data)?;
        Ok(deserialized)
    } else {
        Ok(Vec::new())
    }
}

fn list_notes(note_list: &Vec<data::Note>, num: usize) -> String {
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

fn write_notes(note_list: Vec<data::Note>) {
    serde_json::to_writer(
        &File::create(note_file()).expect("Could not create notes file"),
        &note_list,
    )
    .expect("Could not write to file");
}

fn clear_notes() {
    write_notes(Vec::new());
}

fn set_local() {
    if !PathBuf::from(".notes.json").exists() {
        serde_json::to_writer(
            &File::create(".notes.json").expect("Could not create notes file"),
            &Vec::<data::Note>::new(),
        )
        .expect("Could not write to file");
        println!("Created a local notes file");
    } else {
        println!("Local notes file already exists!")
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
