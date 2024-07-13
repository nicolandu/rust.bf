use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use colored::Colorize;
use home::home_dir;
use serde::{Deserialize, Serialize};

const FILE_NAME: &str = ".todo";

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub label: String,
    pub done: bool,
}

#[derive(Debug)]
pub struct Todo {
    pub tasks: Vec<Entry>,
    pub path: PathBuf,
}

impl Todo {
    pub fn open() -> Result<Self> {
        let dir = home_dir().expect("Could not get home directory");

        let path = dir.join(FILE_NAME);

        // Abort if error, return empty if path doesn't exist
        if !path.try_exists()? {
            return Ok(Self {
                tasks: vec![],
                path,
            });
        }

        // Open the file in read-only mode with buffer.
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let tasks = serde_yaml::from_reader(reader)?;

        Ok(Self { tasks, path })
    }

    pub fn save(&self) -> Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.path)?;
        let writer = BufWriter::new(file);
        serde_yaml::to_writer(writer, &self.tasks)?;
        Ok(())
    }

    pub fn display(&self) {
        for (i, Entry { label, done }) in self.tasks.iter().enumerate() {
            match done {
                true => println!("[{i}] {}", label.strikethrough()),
                false => println!("{}", format!("[{i}] {label}").bold()),
            };
        }
    }

    pub fn add<I>(&mut self, elems: I)
    where
        I: IntoIterator<Item = Entry>,
    {
        self.tasks.extend(elems.into_iter());
    }

    pub fn set_done(&mut self, idxs: &[usize], done: bool) -> Result<()> {
        if idxs.iter().any(|&e| e >= self.tasks.len()) {
            return Err(anyhow!("Index out of bounds"));
        };

        for &i in idxs {
            self.tasks[i].done = done
        }

        Ok(())
    }

    /// O(mn) complexity, based on task list length and idxs length
    pub fn remove(&mut self, idxs: &[usize]) -> Result<()> {
        if idxs.iter().any(|&e| e >= self.tasks.len()) {
            return Err(anyhow!("Index out of bounds"));
        };

        let mut index = 0;
        self.tasks.retain(|_| {
            let ret = !idxs.iter().any(|&i| i == index);
            index += 1;
            ret
        });
        Ok(())
    }
}
