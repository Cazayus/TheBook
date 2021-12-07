use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

const PATH: &str = "target/hello.txt";

fn main() {
    let _v = vec![1, 2, 3];
    // v[99];
    let f: Result<File, std::io::Error> = File::open(PATH);

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(PATH) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let _f = File::open(PATH).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(PATH).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let _f = File::open(PATH).unwrap();
    let _f = File::open(PATH).expect("Failed to open hello.txt");
    // let f = File::open("hello.txt")?;
}

//fn main() -> Result<(), Box<dyn Error>> {
//    let f = File::open("hello.txt")?;

//    Ok(())
//}

fn _read_username_from_file() -> Result<String, io::Error> {
    let f = File::open(PATH);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
fn _read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut f = File::open(PATH)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn _read_username_from_file_even_shorter() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open(PATH)?.read_to_string(&mut s)?;

    Ok(s)
}
fn _read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string(PATH)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
