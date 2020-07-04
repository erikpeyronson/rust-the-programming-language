use std::collections::HashMap;
use std::io;
use std::io::Write;

enum Command {
    Add(String, String),
    List(String),
    ListAll(),
    Help(),
    Exit(),
    None(),
}

struct Company {
    departments: HashMap<String, Department>,
}

impl Company {
    // Create a new company
    fn new() -> Company {
        Company {
            departments: HashMap::new(),
        }
    }

    // Add employe with 'name' to 'department'
    fn add_employee(&mut self, name: String, department: &str) {
        let department = self
            .departments
            .entry(String::from(department))
            .or_insert(Department::new());
        department.add(name);
    }

    // List all departments
    fn list_all(&self) {
        for (name, _) in &self.departments {
            self.list(name);
        }
    }
    // List all employes in department sorted alphabetically
    fn list(&self, department_name: &str) {
        match self.departments.get(department_name) {
            Some(d) => {
                println!("Department {}", department_name);
                d.print()
            }
            None => println!("Department not found {}", department_name),
        }
    }
}

struct Department {
    employees: Vec<String>,
}

impl Department {
    // Create new department
    fn new() -> Department {
        Department {
            employees: Vec::new(),
        }
    }
    // Add employee to department
    fn add(&mut self, name: String) {
        self.employees.push(name);
        self.employees.sort();
    }
    // Print a list of employees
    fn print(&self) {
        for employee in &self.employees {
            println!("{}", employee);
        }
    }
}

fn main() {
    println!("*Employe database started*");
    println!("Enter 'help' for a list of commands or 'exit' to exit");

    let mut company = Company::new();
    loop {
        match read_command() {
            Command::Add(name, department) => company.add_employee(name, &department),
            Command::List(department) => company.list(&department),
            Command::ListAll() => company.list_all(),
            Command::Help() => print_help(),
            Command::None() => continue,
            Command::Exit() => break,
        }
    }
    println!("Program stopped");
}

fn print_help() {
    println!("Available commands:");
    println!("add <name> to <department> - Adds 'name' to 'deparment'");
    println!("remove <name> <department> - Removes 'name' from 'department'");
    println!("list [<department>] 'List employees if optional arg 'department' is omitted print all departments");
    println!("help - print this help info");
    println!("exit - exit the program");
}

fn read_command() -> Command {
    print!("> ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let words: Vec<&str> = input.split_whitespace().collect();

    match words.as_slice() {
        ["add", arg1, "to", arg2] => Command::Add(String::from(*arg1), String::from(*arg2)),
        ["list", arg1] => Command::List(String::from(*arg1)),
        ["list"] => Command::ListAll(),
        ["exit"] => Command::Exit(),
        ["help"] => Command::Help(),
        [] => Command::None(),
        _ => {
            println!("Invalid command. enter 'help' for a list of commands");
            Command::None()
        }
    }
}
