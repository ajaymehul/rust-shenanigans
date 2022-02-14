use std::io::{self, Write};
use std::collections::HashMap;

enum Command {
    Add(String, String),
    List(String),
    Exit,
    Error
}

fn main() {
    let mut database = HashMap::new();
    print_usage();
    loop {
        print!("$ ");
        let _ = io::stdout().flush();
        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read line");
        let cmd: Command = parse_command(&cmd);

        match cmd {
            Command::Add(name, dept) => {
                println!("Adding {} to {}",name,dept);
                database.insert(name, dept);
            }
            Command::List(dept) => {
                let mut employee_list = vec![];
                for (name, department) in &database {
                    
                    if department == &dept {
                        employee_list.push(&name[..])
                    }
                }
                employee_list.sort();
                print_employees(employee_list, &dept);
            }
            Command::Exit => {
                println!("Exiting Employee database....");
                return;
            }
            Command::Error => handle_err()
            
        }
    }

}

fn parse_command(s: &String) -> Command{
    let words: Vec<&str> = s.split(" ").collect();
    let cmd = words.get(0);
    match cmd {
        Some(cmd_) => {
            if cmd_ == &"Add" {
                let name = words.get(1);
                let department = words.get(3);
                match name {
                    Some(name_) => {
                        match department {
                            Some(department_) => {
                                return Command::Add(String::from(*name_), String::from(*department_));
                            }
                            None => return Command::Error
                        }
                    }
                    None => return Command::Error
                }
                
            } else if cmd_ == &"List" {
                let department = words.get(1);
                match department {
                    Some(department_) => {
                        return Command::List(String::from(*department_));
                    }
                    None => return Command::Error
                }
            } else if cmd_ == &"Exit" || cmd_ == &"Exit\n" {
                return Command::Exit
            } else {
                return Command::Error
            }
        }
        None => return Command::Error
    }
}

fn handle_err () {
    println!("Invalid Command"); 
    print_usage();
}


fn print_usage () {
    println!("Usage:");
    println!("Add Employees: Add <name> to <department>");
    println!("List Employees: List <department>");
}

fn print_employees(v: Vec<&str>, department: &str) {
    println!("Employees of {}", department);
    for employee in v {
        println!("{}", employee);
    }
}