use colored::Colorize;
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Employee {
    name: String,
    department: String,
}

#[derive(Debug)]
struct AddCommand {
    name: String,
    department: String,
}
#[derive(Debug)]
enum ListCommand {
    All,
    ByDept { dept: String },
}
#[derive(Debug)]
struct EmployeesDB {
    employees: Vec<Employee>,
    dept_to_employees: HashMap<String, Vec<String>>,
}

impl EmployeesDB {
    fn new() -> Self {
        return EmployeesDB {
            employees: Vec::new(),
            dept_to_employees: HashMap::new(),
        };
    }

    fn add(&mut self, command: AddCommand) {
        let output = format!("Adding {} to {}", command.name, command.department);
        println!("{}", output.green());
        let employee = Employee {
            name: command.name,
            department: command.department,
        };
        let employee_copy = employee.clone();

        self.employees.push(employee);
        let entry = self
            .dept_to_employees
            .entry(employee_copy.department.clone())
            .or_insert(Vec::new());
        entry.push(employee_copy.name);
    }

    fn list(&self, command: ListCommand) {
        match command {
            ListCommand::All => {
                let output = format!("{:?}", self.dept_to_employees);
                println!("{}", output.purple())
            }
            ListCommand::ByDept { dept } => {
                // figure out if this key exists
                match self.dept_to_employees.get(&dept) {
                    Some(employees) => {
                        let output = format!("Our employees in {}: {:?}", dept, employees); // Colorize dept
                        println!("{}", output.purple());
                    }
                    None => {
                        let output = format!("No employees in {}!", dept); // Colorize dept
                        println!("{}", output.purple());
                    }
                }
            }
        }
    }
}

/// Takes in a reference to a command, does case insensitive matching and returns an AddCommand
/// Option based on these rules:
///  - "Add {name} to {department}" => return(Some(AddCommand{name, deparment}))"
///  - return None otherwise
fn parse_add_command(command: &String) -> Option<AddCommand> {
    // check if command is add
    let mut words = command.split(" ").map(|word| word.trim_end());

    // case insensitive matching for the first word
    if !matches!(words.next()?.to_lowercase().as_str(), "add") {
        return None;
    };

    // parse the name of the employee
    let name = words.next()?.to_string();

    // case insensitive matching for preposition
    if !matches!(words.next()?.to_lowercase().as_str(), "to") {
        return None;
    }
    // parse the department of the employee
    let department = words.next()?.to_string();
    let mut chars = department.chars();
    let uppercase_department = match chars.next() {
        Some(first) => {
            // + operator takes a String and a &str and concats the string slice on top (in place)
            first.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str()
        }
        _ => String::new(),
    };

    // we found a match for the add command
    return Some(AddCommand {
        name,
        department: uppercase_department.to_string(),
    });
}

/// Takes in a reference to a command,does case insenitive matching and returns a ListCommand
/// Option based on these rules:
///  - "List all" or "List company" => return Some(ListCommand::All)
///  - "List {department}" => return Some(ListCommand::ByDept{department})
///  -  return None otherwise
fn parse_list_command(command: &String) -> Option<ListCommand> {
    let mut words = command.split(" ").map(|word| word.trim_end());

    if !matches!(words.next()?.to_lowercase().as_str(), "list") {
        return None;
    }

    let department = words.next()?.to_lowercase();
    println!("Department {}", department.green());
    if matches!(department.as_str(), "all") {
        return Some(ListCommand::All);
    }
    let mut chars = department.chars();
    match chars.next() {
        Some(first) => {
            return Some(ListCommand::ByDept {
                // + operator takes a String and a &str and concats the string slice on top (in place)
                dept: first.to_uppercase().collect::<String>()
                    + chars.as_str().to_lowercase().as_str(),
            });
        }
        _ => return None,
    };
}

/// Prints an introduction message about the commands available in the interface
fn print_intro() {
    println!();
    println!("{}", "---------------------------".yellow());
    let intro = format!(
        "{} This is a text interface for adding employees to our database.\n\
        Here are the commands you can use: \n",
        "Hi friend!".bold()
    );
    println!("{}", intro.green());
    println!(
        "{} {} {} {} - {}",
        "Add".green(),
        "<name>".blue().bold(),
        "to".green(),
        "<department>".blue().bold(),
        "adds a new employeed to the database".green()
    );

    println!(
        "{} - {}",
        "List all".green(),
        "lists all employees in the database".green()
    );
    println!(
        "{} {} - {} {}",
        "List".green(),
        "<deptartment>".blue().bold(),
        "lists all employees in ".green(),
        "<deptartment>".blue().bold(),
    );
    println!();
    println!("{}", "Enter your command:".green());
    println!();
}

fn main() {
    print_intro();
    let mut db = EmployeesDB::new();
    loop {
        println!();
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            if input.trim().to_lowercase() == "exit" {
                println!("{}", "Exiting program...".yellow());
                break;
            }
            if let Some(add_command) = parse_add_command(&input) {
                db.add(add_command);
            } else if let Some(list_command) = parse_list_command(&input) {
                db.list(list_command);
            } else {
                println!("{}", "Command not recognized, try again".yellow());
            }
        } else {
            println!("{}", "Error reading user input".red().bold());
        }
    }
}
