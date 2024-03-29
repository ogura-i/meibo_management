use std::{io, num, fmt, error};
use std::fs::File;
use std::io::{BufWriter, Write, BufReader, BufRead};

/*
 * define Date struct
 */

struct Date {
    y: u32,
    m: u8,
    d: u8,
}

/*
 * define method of Date struct
 */
impl Date {
    /*
     * new(): initialize Date struct
     */
    fn new(year: u32, month: u8, day: u8) -> Result<Date, Errors> {
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

    fn parse_date(str: String) -> Result<Date, Errors> {
        let vec: Vec<&str> = str.split('-').collect();

        if vec.len() != 3 {
            Err(Errors::InvalidFormat)
        } else {
            Date::new(vec.get(0).unwrap().parse()?, vec.get(1).unwrap().parse()?, vec.get(2).unwrap().parse()?)
        }
    }

    /*
     * is_valid: check the date is valid?
     */
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

    /*
     * is_leapyear: check the date is leap year
     */
    fn is_leapyear(y: u32) -> bool {
        if ((y % 4 == 0) && (y % 10 != 0)) || (y % 400 == 0) {
            true
        } else {
            false
        }
    }

    /*
     * form_string: Date struct to string
     */
    fn form_string(&self) -> String {
        format!("{}-{}-{}", self.y, self.m, self.d)
    }
}

/*
 * define Profile struct
 */
struct Profile {
    id: u32,
    name: String,
    birth: Date,
    addr: String,
    note: String,
}

/*
 * define method of Profile struct
 */
impl Profile {
    /*
     * new: initialize Profile struct
     */
     fn new(i: u32, na: String, b: String, a: String, no: String) -> Result<Profile, Errors> {
        let profile = Profile {
            id: i,
            name: na,
            birth: Date::parse_date(b)?,
            addr: a,
            note: no,
        };
        Ok(profile)
    }

    fn parse_item(str: &String) -> Result<Profile, Errors> {
        let vec: Vec<&str> = str.trim_end().splitn(5, ',').collect();
        if vec.len() != 5 {
            Err(Errors::InvalidFormat)
        } else {
            let id = vec.get(0).unwrap().parse()?;
            let name = vec.get(1).unwrap().to_string();
            let birth = vec.get(2).unwrap().to_string();
            let addr = vec.get(3).unwrap().to_string();
            let note = vec.get(4).unwrap().to_string();
            Profile::new(id, name, birth, addr, note)
        }
    }
    /*
     * print: print Profile struct
     */
    fn print(&self) {
        println!("Id:    {}", self.id);
        println!("Name:  {}", self.name);
        println!("Birth: {}", self.birth.form_string());
        println!("Addr:  {}", self.addr);
        println!("Note:  {}\n", self.note);
    }

    /*
     * form_csv: Profile struct to csv date
     */
    fn form_csv(&self) -> String {
        let cdata = format!("{},{},{},{},{}", self.id.to_string(), self.name, self.birth.form_string(), self.addr, self.note);
        cdata
    }

    /*
     * find_profile: check profile match word
     */
    fn find_profile(&self, word: & String) -> bool {
        if &self.id.to_string() == word
            || &self.name == word
            || &self.birth.form_string() == word
            || &self.addr == word
            || &self.note == word {
            true
        } else {
            false
        }
    }
}

/*
 * define Errors
 */
#[derive(Debug)]
enum Errors {
    Io(io::Error),
    Parse(num::ParseIntError),
    InvalidValue,
    InvalidFormat,
    NotFound,
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Errors::Io(ref err) => write!(f, "IO error occured: {}", err),
            Errors::Parse(ref err) => write!(f, "Parse error occured: {}", err),
            Errors::InvalidValue => write!(f, "Invalid value"),
            Errors::InvalidFormat => write!(f, "Invalid format"),
            Errors::NotFound => write!(f, "Command not found"),
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
            Errors::NotFound => "Command not found",
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

/*
 * define Command
 */
enum Command {
    Quit,
    Check,
    Print(i32),
    Write(String),
    Read(String),
    Find(String),
    Sort(u32),
}

/*
 * define method of Command
 */
impl Command {
    /*
     * call: each command processing
     */
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
                    profiles.push(Profile::parse_item(&line?)?);
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
                    _ => return Err(Errors::InvalidValue),
                };
            },
        };
        Ok(())
    }
}

/*
 * discrimination: discriminate command 
 */
fn discrimination(args: String, profiles: &mut Vec<Profile>) -> Result<(), Errors>{
    let vec: Vec<&str> = args.splitn(2, " ").collect();
    let command = match vec.first().unwrap().trim_end() {
        "Q" => Command::Quit,
        "C" => Command::Check,
        "P" => {
            match vec.get(1) {
                Some(val) => Command::Print(val.trim_end().parse()?),
                None => return Err(Errors::InvalidFormat),
            }
        },
        "W" => {
            match vec.get(1) {
                Some(val) => Command::Write(val.trim_end().to_string()),
                None => return Err(Errors::InvalidFormat),
            }
        },
        "R" => {
            match vec.get(1) {
                Some(val) => Command::Read(val.trim_end().to_string()),
                None => return Err(Errors::InvalidFormat),
            }
        },
        "F" => {
            match vec.get(1) {
                Some(val) => Command::Find(val.trim_end().to_string()),
                None => return Err(Errors::InvalidFormat),
            }
        },
        "S" => {
            match vec.get(1) {
                Some(val) => Command::Sort(val.trim_end().parse()?),
                None => return Err(Errors::InvalidFormat),
            }
        },
        _ => return Err(Errors::NotFound),
    };
    command.call(profiles)
}

/*
 *
 */
fn parse_line(line: &String, profiles: &mut Vec<Profile>) -> Result<(), Errors>{
    if line.starts_with("%") {
        discrimination(line[1..].to_string(), profiles)?;
    } else {
        profiles.push(Profile::parse_item(line)?);
    }
    Ok(())
}

/*
 * main
 */
fn main() {
    let mut profiles: Vec<Profile> = Vec::new();
    
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok();

        match parse_line(&line, &mut profiles) {
            Ok(_) => {},
            Err(err) => eprintln!("{}", err),
        };
    }
}
