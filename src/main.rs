//! Simulating files one step at a time.

#![allow(unused_variables)]
use rand::prelude::*;
use std::fmt;
use std::fmt::Display;

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

/// Represents a "file"
/// which probably lives on a file system.
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    /// New files are assumed to be empty, but a name is required.
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            data: vec![],
            state: FileState::Closed,
        }
    }

    /// New file with initial data
    pub fn new_with_data(name: &str, data: &Vec<u8>) -> Self {
        Self {
            name: String::from(name),
            data: data.clone(),
            state: FileState::Closed,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }

        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        Ok(read_length)
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

fn open(mut f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_msg = String::from("Permission denied!");
        return Err(err_msg);
    }

    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_msg = String::from("Interrputed by signal!");
        return Err(err_msg);
    }

    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let mut _f1 = File::new("f1.txt");
    let mut f1 = File::new_with_data("f1.txt", &vec![114, 117, 115, 116, 33]);

    let mut buffer: Vec<u8> = vec![];

    f1 = open(f1).unwrap();
    let f1_length = f1.read(&mut buffer).unwrap();
    f1 = close(f1).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f1);
    println!("{}", f1); // Our Display implementation
    println!("{} is {} bytes long", &f1.name, &f1.len());

    println!("{}", text);
}
