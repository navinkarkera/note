use dialoguer::Editor;
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
    List { number: Option<usize> },
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
    let number = if note_list.len() > num {
        num
    } else {
        note_list.len()
    };
    note_list[..number]
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("\n===================================\n\n")
}

fn write_notes(note_list: Vec<data::Note>) {
    serde_json::to_writer(
        &File::create(note_file()).expect("Could not create notes file"),
        &note_list,
    )
    .expect("Could not write to file");
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
