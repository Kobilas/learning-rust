use std::{io, collections::HashMap};

fn main() {
    let mut ans1 = String::new();
    let mut ans2 = String::new();
    // ems - employee management system
    let mut ems = HashMap::new();
    loop {
        print_main_menu();
        get_answer(&mut ans1);
        match &ans1[..] {
            "q" => {
                println!("Thank you for using the Employee Management System");
                return;
            },
            "a" => {
                println!("Enter department to add employee to:");
                get_answer(&mut ans1);
                ems.entry(ans1).or_insert(Vec::new());
                println!("Enter user to add to department {}", ans1);
                get_answer(&mut ans2);
                match ems.get(&ans1) {
                    Some(dept) => dept.push(ans2),
                    None => {
                        println!("Error when getting dept {} from ems", ans1);
                        continue;
                    }
                }
                println!("Added user {} to department {}", ans2, ans1);
            },
            "v" => {
                println!("Current state of ems:");
                println!("{:#?}", ems);
            }
            _ => (),
        }
    }
}

fn print_main_menu() {
    println!("HashMap and Vector Employee Management System");
    println!("                  Main Menu");
    println!("a - Add user to department");
    println!("v - View current status");
    println!("q - Quit");
}

fn get_answer(s: &mut String) {
    io::stdin().read_line(&mut s)
        .expect("Failed to read line");
    *s = s.trim().to_string();
}
