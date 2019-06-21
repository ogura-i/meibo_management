use std::fs::File;
use std::io::{BufWriter, Write, BufReader, BufRead};

struct Profile {
    id: u32,
    name: String,
    birth: String,
    addr: String,
    note: String,
}

impl Profile {
    fn print(&self) {
        println!("Id:    {}", self.id);
        println!("Name:  {}", self.name);
        println!("Birth: {}", self.birth);
        println!("Addr:  {}", self.addr);
        println!("Note:  {}\n", self.note);
    }

    fn form_csv(&self) -> String {
        let cdata = format!("{},{},{},{},{}", self.id.to_string(), self.name, self.birth, self.addr, self.note);
        cdata
    }

    fn find_profile(&self, word: & String) -> bool {
        if &self.id.to_string() == word {
            true
        } else if &self.name == word {
            true
        } else if &self.birth == word {
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
                std::process::exit(1);
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
                    let cdata = profile.form_csv();
                    writeln!(file, "{}", cdata);
                }
            },
            Command::Read(filename) => {
                for cdata in BufReader::new(File::open(filename).unwrap()).lines() {
                    store_data(cdata.unwrap(), profiles)
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
                    3 => profiles.sort_by(|a, b| a.birth.cmp(&b.birth)),
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
    let vec: Vec<&str> = str.trim_end().split(',').collect();
    if vec.len() != 5 {
        println!("error");
    } else {
        let cdata = Profile {
            id: vec[0].parse().unwrap(),
            name: vec[1].to_string(),
            birth: vec[2].to_string(),
            addr: vec[3].to_string(),
            note: vec[4].to_string(),
        };
        profiles.push(cdata);
    }
}

fn main() {
    let mut profiles: Vec<Profile> = Vec::new();
    
    loop {
        let mut str = String::new();
        std::io::stdin().read_line(&mut str).ok();

        if str.starts_with("%") {
            discrimination(str, &mut profiles);
        } else {
            store_data(str, &mut profiles);
        }
    }
}
