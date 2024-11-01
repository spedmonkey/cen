use clap::Error;
use rayon::prelude::*;
//use walkdir::WalkDir;
use polars::prelude::*;
use polars_io::ipc::IpcReader;
use polars_io::ipc::IpcWriter;
use std::fs::File;
use std::{any::Any, fs::DirEntry, path::PathBuf};
//use std::time::Instant;

use clap::Parser;
use jwalk::WalkDir;
use std::env;
use std::path::Path;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    /// Name of the person to greet
    #[arg(short, long, default_value_t = String::from(""))]
    output: String,
    #[arg(short, long, default_value_t = String::from(""))]
    read: String,
    #[arg(short, long, default_value_t = String::from(""))]
    search: String,
}

struct CustomDataFrame {
    dataframe: Result<DataFrame, Error>,
    cache_path: Result<PathBuf, Error>,
    search_word: String,
    print_df: bool,
    write_df: PathBuf,
}

impl CustomDataFrame {
    fn new(
        dataframe: Result<DataFrame, Error>, //dataframe
        cache_path: Result<PathBuf, Error>,  //path to cache
        search_word: String,                 //search word
        print_df: bool,                      //bool whether to print
        write_df: PathBuf,                   //path to write search word data frame to new cache
    ) -> Self {
        Self {
            dataframe,
            cache_path,
            search_word,
            print_df,
            write_df,
        }
    }
    fn set_dataframe() {
        todo!()
    }

    fn set_cache_path() {
        todo!()
    }
    fn set_search_word(&mut self, search_word: String) {
        self.search_word = search_word;
    }
    fn set_print_df(&mut self, print_df: bool) {
        self.print_df = print_df;
    }

    fn set_write_df(&mut self, write_df: PathBuf) {
        self.write_df = write_df;
    }
}

impl Default for CustomDataFrame {
    fn default() -> Self {
        Self {
            //name: String::from("Anonymous"),
            //age: 30,
            dataframe: Err(std::fmt::Error.into()),
            cache_path: Err(std::fmt::Error.into()),
            search_word: String::from(""),
            print_df: false,
            write_df: PathBuf::new(),
        }
    }
}

fn write(mut df: DataFrame, path: Result<PathBuf, Error>) {
    let mut file = File::create(path.unwrap()).expect("could not create file");

    let _ = IpcWriter::new(&mut file)
        .finish(&mut df)
        .map_err(|e| PolarsError::ComputeError(format!("awosome: {}", e).into()));

    println!("{:?}", df);
}

fn read_cache(path: PathBuf) -> PolarsResult<DataFrame> {
    println!("{:?}", path.as_path());
    let file = File::open(path.as_path()).expect("file not found");

    let df = IpcReader::new(file)
        .finish()
        .map_err(|e| PolarsError::ComputeError(format!("my awesome error: {}", e).into()))?;

    Ok(df)
}

fn read_dir(dir: String) -> PolarsResult<DataFrame> {
    let root_dir = dir; // Replace with your target directory

    println!("Starting search");
    let start = Instant::now();
    // Traverse the directory recursively
    let entries: Vec<_> = WalkDir::new(root_dir)
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
    result.push(series_file_names);
    result.push(series_paths);

    let duration = start.elapsed();
    println!("duration: {:?} seconds", (duration.as_secs() as f32) / 60.0);

    let df: DataFrame = DataFrame::new(result)?;

    if df.is_empty() {
        Err(PolarsError::ComputeError("An error occurred".into()))
    } else {
        Ok(df)
    }
}
/*
fn online() {
    let mut result_df = DataFrame::empty();
    let mut df = getdata();

    match df {
        Ok(df) => write(df),
        Err(e) => eprintln!("Error creating Dataframe: {:?}", e),
    }
}


fn offline() -> PolarsResult<DataFrame> {
    let _start = Instant::now();
    let df = read();
    let _duration = _start.elapsed();
    println!("duration: {:?} milli seconds", (_duration.as_millis()));
    match df {
        Ok(df) => return Ok(df),
        Err(e) => return Err(e),
    }
}
     */
fn cache_path() -> Result<PathBuf, Error> {
    let mut current_exe_path = env::current_exe()?;
    let a = current_exe_path.parent().unwrap().join("cache.tpc");
    println!("cache path: {:?}", a);
    if a.exists() {
        ()
    } else {
        File::create(a.clone());
    }
    Ok(a)
}

fn main() {
    //online();

    //let df = offline();
    //let mut data = df.unwrap();

    let args = Args::parse();
    let output = args.output;
    let read = args.read;
    let search = args.search;

    let mut df = DataFrame::empty();
    let cache = cache_path();

    let mut my_data = CustomDataFrame::default();

    my_data.set_print_df(true);
    my_data.set_write_df(Path::new(output.as_str()).to_path_buf());
    my_data.set_search_word(String::from("my awesome words"));

    println!("my output: {:?}", my_data.write_df);

    if read != String::from("") {
        let result = read_dir(read);
        match result {
            Ok(data) => println!("{:?}", data),
            Err(df) => panic!("failed to get data frame"),
        }
    }

    //write(df, path);

    if cache.is_ok() {
        let temp = read_cache(cache.unwrap());
        match temp {
            Ok(a) => df = a,
            Err(e) => println!("line 156: {:?}", e),
        }
    }

    if search != String::from("") {
        let jpg_indexes = df
            .column("file names")
            .unwrap()
            .iter()
            .enumerate()
            .filter_map(|(index, value)| {
                if value.to_string().to_lowercase().contains(&search) {
                    Some(index)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for row in jpg_indexes {
            let my_row = df.get_row(row).unwrap();
            println!("{:?}", my_row);
        }
    }

    if output != String::from("") {
        todo!();
    }
}

/*



fn main() {
    let file_path = Path::new("example.txt");

    if file_path.exists() {
        println!("File exists.");
    } else {
        println!("File does not exist.");
    }
}

*/

//logic flow

//does ipc cache file exist?
//if ipc file doesn't exist create ipc file

//is dir specificied
//if dir is specified iterate dir

//create dataframe
//save dataframe to ipc file

//load ipc file

//if search is specificed
//search ipc file
//create dataframe
//output dataframe to human readable text

//print ipc file to shell
