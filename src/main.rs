use std::collections::HashMap;
use std::ffi::OsString;
use std::io::{BufReader, Error, Read};
use std::fs::File;
use std::iter;
use std::path::{Path, PathBuf};
use clap::{Parser};
use ring::digest::{Context, Digest, SHA256};
use data_encoding::HEXUPPER;
use ansi_term::Colour::Red;
use unicode_segmentation::UnicodeSegmentation;

/// Simple command line tool to compare the contents of the given folders
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// A list of multiple directories
    #[structopt(required = true, min_values = 1)]
    #[clap(parse(from_os_str))]
    directories: Vec<PathBuf>,

    /// Optional filter: only regard files with this extension
    #[clap(long)]
    extension: Option<OsString>,

    /// The width of each column in the output ASCII table
    #[clap(long, default_value_t=20)]
    colwidth: usize,

    /// With this flag, only the differences are listed, i.e. only those files are outputted that
    /// either (a) don't occur in all folders, or (b) don't have the same name in all folders, or
    /// (c) occur more than once in at least one folder
    #[clap(long)]
    diffonly: bool,
}

fn main() {
    let args = Args::parse();

    // This map maps each hash to all the files in the folders that have that hash/digest:
    let mut hash_to_files: HashMap<String, Vec<PathBuf>> = HashMap::new();

    // For each directory and for each file in each directory, update `hash_to_files`:
    for directory in args.directories.iter() {
        if !directory.is_dir() {
            eprintln!("{}", Red.paint(format!("Error: {} is not a directory!", directory.display())));
        } else {
            match directory.read_dir() {
                Ok(read_dir) => {
                    for file in read_dir {
                        match file {
                            Ok(file) => {
                                let file: PathBuf = file.path(); // Turn a DirEntry into a PathBuf.
                                if args.extension == None || args.extension == file.extension().map(|os_str| os_str.to_os_string()) {
                                    match file_hash(&file) {
                                        Ok(hash) => hash_to_files.entry(hash).or_insert(Vec::new()).push(file),
                                        Err(error) => eprintln!("{}", Red.paint(format!("Error: An error occurred while hashing {}: {}", file.display(), error)))
                                    }
                                }
                            },
                            Err(error) => eprintln!("{}", Red.paint(format!("Error: An IO error occurred while iterating through {}: {}", directory.display(), error)))
                        }
                    }
                },
                Err(error) => eprintln!("{}", Red.paint(format!("Error: Directory {} could not be read: {}", directory.display(), error)))
            }
        }
    }

    // Print out the result as an ASCII table:
    println!(); // Put a newline over and under the ASCII table to make it more readable!
    println!("#\tSHA256{}\t{}",
             " ".repeat(64 - "SHA256".len()),
             args.directories.iter().map(|dir: &PathBuf| fixed_length(dir.file_name().map( |os_str| os_str.to_str()).flatten().unwrap_or("???"), args.colwidth, " ")).collect::<Vec<String>>().join("\t")
    );

    // Turn the HashMap into a Vec to be able to sort it by hash:
    let mut hash_to_files: Vec<(String, Vec<PathBuf>)> = hash_to_files.into_iter().collect::<Vec<(String, Vec<PathBuf>)>>();
    hash_to_files.sort_unstable_by(|(hash1, _), (hash2, _)| hash1.cmp(&hash2)); // sort_unstable_by_key would require inefficient cloning!
    // Note: without sorting, the order of the output would be different everytime – which is not that nice.

    // Take care of the "--diffonly" flag, if it is set:
    if args.diffonly {
        // Retain only those listings for files/hashes that...
        hash_to_files.retain(|(_hash, files)|
            files.len() != args.directories.len() // ...either (a) don't occur in all folders, or (c) occur more than once in at least one folder
            || !files.iter().all(|file| file.file_name() == files[0].file_name()) // ...or (b) don't have the same name in all folders
            // Note: `files[0]` won't panic because files.len() == args.directories.len() >= 1
            // Note: When the name of two files is equal, they have to be in different folders!
        );
    }

    // Do the actual print-out:
    let mut counter = 1;
    for (hash, files) in hash_to_files {
        println!("{}\t{}\t{}",
                 counter,
                 hash,
                 args.directories.iter()
                     // Map each directory to all the files in it with the hash `hash` ("in it" meaning directly in it, i.e. not within sub-folders!):
                     .map(|dir| files.iter().filter(|file| file.parent().unwrap() == dir).collect::<Vec<&PathBuf>>()) // file.starts_with(dir) would also allow the file to be in a subdir of dir!
                     // Map 0 files to "–", map 1 file to its file name, map 2+ files to "(X files)":
                     .map(|files| match files.len() {
                         0 => "–".to_string(),
                         1 => files[0].file_name().map(|file_name| file_name.to_str()).flatten().unwrap_or("(1 file)").to_string(),
                         n => format!("({} files)", n)
                     })
                     .map(|column_string: String| fixed_length(&column_string, args.colwidth, " "))
                     .collect::<Vec<String>>().join("\t")
        );
        counter += 1;
    }
    println!();
}

/// Takes a String `s` and makes it have a fixed length `len`.
/// When `s` is longer than `len`, it is cut off.
/// When `s` is shorter than `len`, the `padding` character is appended n times.
fn fixed_length(s: &str, len: usize, padding: &str) -> String {
    s.graphemes(true).chain(iter::repeat(padding)).take(len).collect::<String>()
    // format!("{: <32}", s) is an alternative way of padding (but it does not cut it off when it's longer!)
}

/// Hashes the content of a given file (computes the digest).
fn file_hash<P: AsRef<Path>>(file_path: P) -> Result<String, Error> {
    // cf. https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html
    let file: File = File::open(file_path)?;
    let reader: BufReader<File> = BufReader::new(file);
    let digest: Digest = sha256_digest(reader)?;
    Ok(HEXUPPER.encode(digest.as_ref()))
}

/// Copied from https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html
fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Error> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}