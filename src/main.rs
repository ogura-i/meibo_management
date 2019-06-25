use std::fs::File;
use std::io::{BufWriter, Write, BufReader, BufRead};

struct Date {
    y: u32,
    m: u8,
    d: u8,
}

impl Date {
    fn new(str: String) -> Option<Date> {
        let vec: Vec<&str> = str.split('-').collect();
        
        if vec.len() != 3 {
            None
        } else {
            if Date::is_valid(vec[0].parse().unwrap(), vec[1].parse().unwrap(), vec[2].parse().unwrap()) {
                let date = Date {
                    y: vec[0].parse().unwrap(),
                    m: vec[1].parse().unwrap(),
                    d: vec[2].parse().unwrap(),
                };
                Some(date)
            } else {
                None
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
    fn call(&self, profiles: &mut Vec<Profile>) {
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
                let mut file = BufWriter::new(File::create(filename).unwrap());
                for profile in profiles.iter() {
                    let line = profile.form_csv();
                    writeln!(file, "{}", line);
                }
            },

            Command::Read(filename) => {
                for line in BufReader::new(File::open(filename).unwrap()).lines() {
                    store_data(line.unwrap(), profiles)
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
                    1 => profiles.sort_by_cached_key(|x| x.id),
                    2 => profiles.sort_by(|a, b| a.name.cmp(&b.name)),
                    3 => profiles.sort_by(|a, b| a.birth.form_string().cmp(&b.birth.form_string())),
                    4 => profiles.sort_by(|a, b| a.addr.cmp(&b.addr)),
                    5 => profiles.sort_by(|a, b| a.note.cmp(&b.note)),
                    _ => {},
                };
            },

            Command::Notfound => println!("command not found"),
        };
    }
}

fn discrimination(args: String, profiles: &mut Vec<Profile>) {
    let vec: Vec<&str> = args.split(" ").collect();
    let command = match vec.first().unwrap().trim_end() {
        "%Q" => Command::Quit,
        "%C" => Command::Check,
        "%P" => Command::Print(vec.get(1).unwrap().trim_end().parse().unwrap()),
        "%W" => {
            Command::Write(vec.get(1).unwrap().trim_end().to_string())
        },
        "%R" => {
            Command::Read(vec.get(1).unwrap().trim_end().to_string())
        },
        "%F" => {
           Command::Find(vec.get(1).unwrap().trim_end().to_string())
        },
        "%S" => {
           Command::Sort(vec.get(1).unwrap().trim_end().parse().unwrap())
        },
        _ => Command::Notfound,
    };
    command.call(profiles)
}

// FIXME: add a error handling
fn store_data(str: String, profiles: &mut Vec<Profile>) {
    let vec: Vec<&str> = str.trim_end().splitn(5, ',').collect();
    if vec.len() != 5 {
        println!("error");
    } else {
        let profile = Profile {
            id: vec[0].parse().unwrap(),
            name: vec[1].to_string(),
            birth: Date::new(vec[2].to_string()).unwrap(),
            addr: vec[3].to_string(),
            note: vec[4].to_string(),
        };
        profiles.push(profile);
    }
}

fn main() {
    let mut profiles: Vec<Profile> = Vec::new();
    
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).ok();

        if line.starts_with("%") {
            discrimination(line, &mut profiles);
        } else {
            store_data(line, &mut profiles);
        }
    }
}
