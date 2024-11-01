use std::any::Any;
use std::fmt::Error;
use std::io::{self, Read};
use std::path::PathBuf;

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

use regex::{Captures, Regex};

#[derive(Debug)]

struct Cache {
    cache_path: Option<PathBuf>,
    cache: Option<String>,
}

impl Cache {
    pub fn new() -> Self {
        Cache::default()
    }
}

impl Default for Cache {
    fn default() -> Self {
        Cache {
            cache_path: Some(PathBuf::new()),
            cache: None,
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
    let re = Regex::new(r"(--\w*)(.*)").unwrap();
    loop {
        println!("type action to perfom (--cache, --write, --print)");
        let readline: std::result::Result<String, ReadlineError> = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let command: std::result::Result<(String, String), &str> =
                    get_command(re.clone(), line.clone());
                my_print(command, &mut cache_object);

                let _ = rl.add_history_entry(line.as_str());
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

fn my_print(command: std::result::Result<(String, String), &str>, mut cache_object: &mut Cache) {
    match command {
        Ok((_, _)) => get_action(
            command.clone().unwrap().0.as_str(),
            command.unwrap().1.as_str(),
            cache_object,
        ),

        Err(e) => println!("failed to get command with error: {}", e),
    }
}

fn get_action(command: &str, argument: &str, mut cache_object: &mut Cache) {
    match command {
        "--print" | "--search" | "--write" => printer(command, argument, cache_object),
        "--cache" => cache(argument, cache_object),
        //"--print" => println!("printing"),
        //"--search" => println!("searching"),
        //"--write" => println!("writing"),
        _ => println!("Unknown command: commands are (--cache, --print, --search, --write)"),
    }
}

fn printer(command: &str, argument: &str, cache_object: &Cache) {
    println!(
        "command:{} argument: {} cache_object {:?}",
        command, argument, cache_object
    );
}

fn get_command(re: Regex, line: String) -> std::result::Result<(String, String), &'static str> {
    let captures_stack = re.captures(line.as_str());
    println!("captures stack: {:?}", captures_stack);
    match captures_stack {
        Some(captures) => Ok((captures[1].to_string(), captures[2].to_string())),
        None => Err("failed to get command"),
    }
}

fn cache(cach_path: &str, mut cache_object: &mut Cache) {
    println!("cache path: {:?}", cach_path);
    let file = PathBuf::from(cach_path.trim());
    println!("file: {:?}", file);

    cache_object.cache_path = Some(file.clone());

    //let path = PathBuf::from(
    //    "/dsms/src105/weta/dev/user/crussell/shots/scripts/rust/repltest/src/main.rs",
    //);
    //cache_object.cache_path = Ok(path);
    println!("file exists: {}", file.exists());
}

fn write() {
    println!("write my file path");
}

//actions

//cache directory
//print cache
//write cache to custom cache.tpc
//search loaded cache
//output search to custom cache
//load existing cache.tpc
