use std::{io, num, fmt, error};
use std::fs::File;
use std::io::{BufWriter, Write, BufReader, BufRead};

struct Date {
    y: u32,
    m: u8,
    d: u8,
}

impl Date {
    fn new(str: String) -> Result<Date, Errors> {
        let vec: Vec<&str> = str.split('-').collect();

        if vec.len() != 3 {
            Err(Errors::InvalidFormat)
        } else {
            let year = vec.get(0).unwrap().parse()?;
            let month = vec.get(1).unwrap().parse()?;
            let day = vec.get(2).unwrap().parse()?;

            if Date::is_valid(year, month, day) {
                let date = Date {
                    y: year,
                    m: month,
                    d: day,
                };
                Ok(date)
            } else {
                Err(Errors::InvalidValue)
            }
        }
    }

    fn is_valid(y: u32, m: u8, d: u8) -> bool {
        let days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        if (y == 0) || (m == 0) || (m > 12) {
            false
        } else {
            let lday = if (m == 2) && Date::is_leapyear(y) {
                29
            } else {
                days[(m as usize) - 1]
            };
            if (d == 0) || (d > lday) {
                false
            } else {
                true
            }
        }
    }

    fn is_leapyear(y: u32) -> bool {
        if ((y % 4 == 0) && (y % 10 != 0)) || (y % 400 == 0) {
            true
        } else {
            false
        }
    }

    fn form_string(&self) -> String {
        format!("{}-{}-{}", self.y, self.m, self.d)
    }
}

struct Profile {
    id: u32,
    name: String,
    birth: Date,
    addr: String,
    note: String,
}

impl Profile {
    fn print(&self) {
        println!("Id:    {}", self.id);
        println!("Name:  {}", self.name);
        println!("Birth: {}", self.birth.form_string());
        println!("Addr:  {}", self.addr);
        println!("Note:  {}\n", self.note);
    }

    fn form_csv(&self) -> String {
        let cdata = format!("{},{},{},{},{}", self.id.to_string(), self.name, self.birth.form_string(), self.addr, self.note);
        cdata
    }

    fn find_profile(&self, word: & String) -> bool {
        if &self.id.to_string() == word {
            true
        } else if &self.name == word {
            true
        } else if &self.birth.form_string() == word {
            true
        } else if &self.addr == word {
            true
        } else if &self.note == word {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
enum Errors {
    Io(io::Error),
    Parse(num::ParseIntError),
    InvalidValue,
    InvalidFormat
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Errors::Io(ref err) => write!(f, "IO error occured: {}", err),
            Errors::Parse(ref err) => write!(f, "Parse error occured: {}", err),
            Errors::InvalidValue => write!(f, "Invalid value"),
            Errors::InvalidFormat => write!(f, "Invalid format"),
        }
    }
}

impl error::Error for Errors {
    fn description(&self) -> &str {
        match *self {
            Errors::Io(ref err) => err.description(),
            Errors::Parse(ref err) => err.description(),
            Errors::InvalidValue => "Invalid value",
            Errors::InvalidFormat => "Invalid format",
        }
    }
}

impl From<io::Error> for Errors {
    fn from(err: io::Error) -> Errors {
        Errors::Io(err)
    }
}

impl From<num::ParseIntError> for Errors {
    fn from(err: num::ParseIntError) -> Errors {
        Errors::Parse(err)
    }
}

enum Command {
    Quit,
    Check,
    Print(i32),
    Write(String),
    Read(String),
    Find(String),
    Sort(u32),
    Notfound,
}

impl Command {
    fn call(&self, profiles: &mut Vec<Profile>) -> Result<(), Errors> {
        match self {
            Command::Quit => {
                std::process::exit(0);
            },

            Command::Check => {
                println!("{} profile(s).", profiles.len());
            },

            Command::Print(num) => {
                if *num > 0 {
                    for profile in profiles.iter().take(*num as usize) {
                        profile.print();
                    }
                } else if *num == 0 {
                    for profile in profiles.iter() {
                        profile.print();
                    }
                } else {
                    let end = profiles.len() as i32;
                    for i in end+*num..end {
                        profiles[i as usize].print();
                    }
                }
            },

            Command::Write(filename) => {
                let mut file = BufWriter::new(File::create(filename)?);
                for profile in profiles.iter() {
                    let line = profile.form_csv();
                    writeln!(file, "{}", line)?;
                }
            },

            Command::Read(filename) => {
                for line in BufReader::new(File::open(filename)?).lines() {
                    store_data(line.unwrap(), profiles)?
                }
            },

            Command::Find(word) => {
                for profile in profiles.iter() {
                    if profile.find_profile(word) {
                        profile.print();
                    }
                }
            },

            Command::Sort(num) => {
                match num {
                    1 => profiles.sort_by_key(|x| x.id),
                    2 => profiles.sort_by(|a, b| a.name.cmp(&b.name)),
                    3 => profiles.sort_by(|a, b| a.birth.form_string().cmp(&b.birth.form_string())),
                    4 => profiles.sort_by(|a, b| a.addr.cmp(&b.addr)),
                    5 => profiles.sort_by(|a, b| a.note.cmp(&b.note)),
                    _ => {},
                };
            },

            Command::Notfound => println!("command not found"),
        };
        Ok(())
    }
}

fn discrimination(args: String, profiles: &mut Vec<Profile>) -> Result<(), Errors>{
    let vec: Vec<&str> = args.split(" ").collect();
    let command = match vec.first().unwrap().trim_end() {
        "%Q" => Command::Quit,
        "%C" => Command::Check,
        "%P" => {
            match vec.get(1) {
                Some(val) => Command::Print(val.trim_end().parse()?),
                None => return Err(Errors::InvalidFormat),
            }
        },
        "%W" => {
            match vec.get(1) {
                Some(val) => Command::Write(val.trim_end().to_string()),
                None => return Err(Errors::InvalidFormat),
            }
        },
        "%R" => {
            match vec.get(1) {
                Some(val) => Command::Read(val.trim_end().to_string()),
                None => return Err(Errors::InvalidFormat),
            }
        },
        "%F" => {
            match vec.get(1) {
                Some(val) => Command::Find(val.trim_end().to_string()),
                None => return Err(Errors::InvalidFormat),
            }
        },
        "%S" => {
            match vec.get(1) {
                Some(val) => Command::Sort(val.trim_end().parse()?),
                None => return Err(Errors::InvalidFormat),
            }
        },
        _ => Command::Notfound,
    };
    command.call(profiles)
}

fn store_data(str: String, profiles: &mut Vec<Profile>) -> Result<(), Errors> {
    let vec: Vec<&str> = str.trim_end().splitn(5, ',').collect();
    if vec.len() != 5 {
        Err(Errors::InvalidFormat)
    } else {
        let profile = Profile {
            id: vec[0].parse()?,
            name: vec[1].to_string(),
            birth: Date::new(vec[2].to_string())?,
            addr: vec[3].to_string(),
            note: vec[4].to_string(),
        };
        profiles.push(profile);
        Ok(())
    }
}

fn main() {
    let mut profiles: Vec<Profile> = Vec::new();
    
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok();

        if line.starts_with("%") {
            match discrimination(line, &mut profiles) {
                Ok(_) => {},
                Err(err) => eprintln!("{}", err),
            };
        } else {
            match store_data(line, &mut profiles) {
                Ok(_) => {},
                Err(err) => eprintln!("{}", err),
            };
        }
    }
}
