use std::fmt::Error;
use std::fs::read_dir;
use std::io::{self, Read};

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use crossterm::style::Stylize;

use regex::{Captures, Regex};

use std::io::{stdout, Write};

use rayon::prelude::*;
//use walkdir::WalkDir;
use polars::prelude::*;
use polars_io::ipc::IpcReader;
use polars_io::ipc::IpcWriter;
use std::fs::File;
use std::{any::Any, fs::DirEntry, path::PathBuf};
//use std::time::Instant;

use jwalk::WalkDir;
use std::env;
use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]

struct Cache {
    cache_path: Option<PathBuf>,
    cache: Option<String>,
    regex: Regex,
    command: std::result::Result<(String, String), &'static str>,
}

impl Cache {
    pub fn new() -> Self {
        Cache::default()
    }
    pub fn get_command(&mut self, line: String) {
        let captures_stack = self.regex.captures(line.as_str());
        match captures_stack {
            Some(captures) => self.command = Ok((captures[1].to_string(), captures[2].to_string())),
            None => self.command = Err("failed to get command"),
        }
    }

    fn my_print(&mut self) {
        match &self.command {
            Ok((_, _)) => self.get_action(
                self.command.clone().unwrap().0.as_str(),
                self.command.clone().unwrap().1.as_str(),
            ),

            Err(e) => println!("failed to get command with error: {}", e),
        }
    }

    fn get_action(&mut self, command: &str, argument: &str) {
        match command {
            "--print" | "--search" | "--write" => self.printer(command, argument),
            "--cache" => self.cache(argument),
            //"--print" => println!("printing"),
            //"--search" => println!("searching"),
            //"--write" => println!("writing"),
            _ => println!("Unknown command: commands are (--cache, --print, --search, --write)"),
        }
    }

    fn cache(&mut self, cach_path: &str) {
        let mut cache_dir = PathBuf::from(cach_path.trim());

        if cache_dir.is_dir() {
            println!(
                "{}",
                (format!("Setting Cache path to {:?}", cache_dir).green())
            );
            self.cache_path = Some(cache_dir.clone());

            let a = self.clone().read_directory(cache_dir.as_path());
            match a {
                Ok(cache) => println!("{:?}", cache),
                _ => println!("failed"),
            }
        } else {
            //println!("{}No cache path set, file not found", color::Fg(color::Red));

            println!("{}", "Failed to set cache path no file found.".red());
        }

        //let path = PathBuf::from(
        //    "/dsms/src105/weta/dev/user/crussell/shots/scripts/rust/repltest/src/main.rs",
        //);
        //cache_object.cache_path = Ok(path);
        println!("{:?}", self);
    }

    fn printer(&self, command: &str, argument: &str) {
        println!(
            "{}",
            format!(
                "command:{} argument: {} cache_object {:?}",
                command, argument, self
            )
            .blue()
        );
    }

    fn write() {
        println!("write my file path");
    }

    pub fn read_directory(self, dir: &Path) -> PolarsResult<DataFrame> {
        //let root_dir = dir.to_string(); // Replace with your target directory

        println!("Starting search");
        let start = Instant::now();
        // Traverse the directory recursively
        let entries: Vec<_> = WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|a| a.path())
            .collect::<Vec<_>>();
        println!("entries: {:?}", entries.len());

        let file_names = entries
            .iter()
            .map(|x| String::from(x.file_name().unwrap().to_str().unwrap()))
            .collect::<Vec<String>>();
        let paths = entries
            .iter()
            .map(|x| String::from(x.as_path().to_str().unwrap()))
            .collect::<Vec<String>>();

        let series_file_names = Series::new("file names".into(), file_names);
        let series_paths = Series::new("paths".into(), paths);

        let mut result = Vec::new();
        result.push(polars::prelude::Column::Series(series_file_names));
        result.push(polars::prelude::Column::Series(series_paths));

        let duration = start.elapsed();
        println!("duration: {:?} seconds", (duration.as_secs() as f32) / 60.0);

        let df: DataFrame = DataFrame::new(result)?;

        if df.is_empty() {
            Err(PolarsError::ComputeError("An error occurred".into()))
        } else {
            Ok(df)
        }
    }
}

impl Default for Cache {
    fn default() -> Self {
        Cache {
            cache_path: Some(PathBuf::new()),
            cache: None,
            regex: Regex::new(r"(--\w*)(.*)").unwrap(),
            command: Err("No current command"),
        }
    }
}

fn main() -> Result<()> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    let mut cache_object = Cache::new();
    //let re = Regex::new(r"(--\w*)\s+(.*)").unwrap();
    //let re = Regex::new(r"(--\w*)(.*)").unwrap();
    loop {
        println!("type action to perfom (--cache, --write, --print) \n");
        let readline: std::result::Result<String, ReadlineError> = rl.readline(">> ");
        match readline {
            Ok(line) => {
                //get_command(line.clone(), &cache_object);
                cache_object.get_command(line.clone());
                cache_object.my_print();
                //cache_object.my_print();

                let _ = rl.add_history_entry(line.clone().as_str());
                println!("\n")
                //println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())
}

//actions
//cache directory
//print cache
//write cache to custom cache.tpc
//search loaded cache
//output search to custom cache
//load existing cache.tpc
