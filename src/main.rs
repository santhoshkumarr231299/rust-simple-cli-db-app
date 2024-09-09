use std::{ error, io::{self, Write} };

use ::mysql::{params, prelude::Queryable};

mod mysql;

fn main() {
   
    loop {
        println!("Welcome to Student Dashboard");
        println!("1 -> Login");
        println!("2 -> Signup");
        println!("3 -> Exit");
        print!("Select one of the choices : ");
        io::stdout().flush().expect("Error");

        let mut str = String::new();
        let num = get_input_i32();

        match num {
            Ok(n) => {
                match n {
                    1 => { login(); }
                    2 => { create_user(); }
                    3 => { return; }
                    _ => { println!("Invalid Input") }
                }
             }
            Err(e) => { eprintln!("Error : {}", e) }
        }
    }
}

fn login() {
    println!("Login : ");

    let mut username: String = String::new();
    let mut password: String = String::new();

    print!("Username : ");
    io::stdout().flush().expect("Error");
    username = get_input_string();
    print!("Password : ");
    io::stdout().flush().expect("Error");
    password = get_input_string();

    if validate_user(username, password) {
        println!("Logged In Successfully");
        loop {
            println!("1 -> Show All Students");
            println!("2 -> Add a Student");
            println!("3 -> Logout");
            print!("Enter your choice : ");
            io::stdout().flush().expect("Error");
            let choice = get_input_i32();

            match choice {
                Ok(n) => {
                    match n {
                        1 => display_all_students(),
                        2 => add_a_student(),
                        3 => {
                            println!("Logged out successfully");
                            break;
                        },
                        _ => println!("Invalid Input")
                    }
                }
                Err(er) => println!("Error in displaying the students")
            }

        }
    } else {
        println!("Username and Password Mismatch");
    }   

}

fn validate_user(username : String, password: String) -> bool {
    let mut con = mysql::get_connection();
    let users: Option<String> = con.exec_first(r"select username from users where username = :username and password = :password", params! {
        "username" => username,
        "password" => password
    }).expect("Failed");

    return users.is_some();
}

fn create_user() {
    print!("New Username : ");
    io::stdout().flush().expect("Error");
    let username = get_input_string();

    print!("New Password : ");
    io::stdout().flush().expect("Error");
    let password: String = get_input_string();

    let mut con = mysql::get_connection();
    con.exec_drop(r"insert into users (username, password) values (:username, :password)", params! {
        "username" => username,
        "password" => password
    }).unwrap();

    println!("New User created successfully");
}

fn display_all_students() {
    println!("Displaying all the student details");
    let mut con = mysql::get_connection();
    let users: Vec<(String, String)> = con.query(r"select id, name from students").unwrap();

    for (id, name) in users {
        println!("Username : {} ID: {}", id, name);
    }

}

fn add_a_student() {
    print!("Name of the student : ");
    io::stdout().flush().expect("Error");
    let name = get_input_string();

    print!("ID of the student : ");
    io::stdout().flush().expect("Error");
    let id = get_input_string();

    let mut con = mysql::get_connection();
    con.exec_drop(r"insert into students (name, id) values (:name, :id)", params! {
        "name" => name,
        "id" => id
    }).unwrap();

    println!("Student details successfully added");
}

fn get_input_i32() -> Result<i32, std::num::ParseIntError> {
    let mut str = get_input_string();
    let num: Result<i32, _> = str.trim().parse();
    return num;
}

fn get_input_string() -> String {
    let mut str = String::new();
    io::stdin().read_line(&mut str);
    return str;
}