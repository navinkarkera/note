use chrono::prelude::*;
use colored::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fmt, fs};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    datetime: DateTime<Local>,
    directory: PathBuf,
    title: String,
    note: String,
}

impl Note {
    pub fn create(directory: PathBuf, note: String) -> Note {
        let mut title = note.lines().next().unwrap().to_owned();
        title = title.split("# ").collect();
        Note {
            datetime: Local::now(),
            directory,
            title,
            note,
        }
    }

    pub fn make_markdown(&self) -> String {
        format!(
            "---\ntitle: {}\ndate: {}\n\n---\n\n{}",
            &self.title, &self.datetime, &self.note
        )
    }

    pub fn update_summary(&self) -> std::io::Result<()> {
        if let Ok(summary_file) = fs::read_to_string(&self.directory.join("SUMMARY.md")) {
            let new_summary = format!(
                "{}\n- [{}]({}.md)",
                summary_file,
                &self.title,
                &self
                    .title
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join("_")
            );
            fs::write(&self.directory.join("SUMMARY.md"), new_summary)?
        }
        Ok(())
    }
    pub fn save(&self) -> std::io::Result<()> {
        let note_path = &self.directory.join(format!(
            "{}.md",
            &self
                .title
                .split_whitespace()
                .collect::<Vec<&str>>()
                .join("_")
        ));
        fs::write(note_path, &self.make_markdown())?;
        self.update_summary()
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n\n{} {}",
            self.make_markdown().cyan(),
            "Created at:".green(),
            self.datetime.format("%v %r").to_string().magenta()
        )
    }
}
