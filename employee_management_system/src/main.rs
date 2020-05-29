use std::{io, collections::{HashMap, hash_map::Entry}};

fn main() {
    let mut ans1 = String::new();
    let mut ans2 = String::new();
    // ems - employee management system
    let mut ems = HashMap::new();
    loop {
        print_main_menu();
        get_answer(&mut ans1);
        /*
         *io::stdin().read_line(&mut ans1)
         *    .expect("Failed to read line");
         *ans1 = ans1.trim().to_string();
         */
         match &ans1[..] {
            "q" => {
                println!("Thank you for using the Employee Management System");
                return;
            },
            "a" => {
                ans1.clear();
                println!("Enter department to add employee to:");
                get_answer(&mut ans1);
                /*
                 *io::stdin().read_line(&mut ans1)
                 *    .expect("Failed to read line");
                 *ans1 = ans1.trim().to_string();
                 */
                //ems.entry(ans1).or_insert(Vec::new());
                println!("Enter user to add to department {}", ans1);
                get_answer(&mut ans2);
                println!("Adding user {} to department {}", ans2, ans1);
                /*
                 *io::stdin().read_line(&mut ans2)
                 *    .expect("Failed to read line");
                 *ans2 = ans2.trim().to_string();
                 */
                /* Previous attempt
                 *match ems.get(&ans1) {
                 *    Some(mut dept) => dept.push(&ans2),
                 *    None => {
                 *        println!("Error when getting dept {} from ems", ans1);
                 *        continue;
                 *    }
                 *}
                 */
                match ems.entry(ans1) {
                    Entry::Vacant(e) => {
                        e.insert(vec![ans2]);
                        //ans1 = e.key().to_string();
                    },
                    Entry::Occupied(mut e) => {
                        e.get_mut().push(ans2);
                        //ans1 = e.key().to_string();
                    },
                }
                //println!("Added user {} to department {}", ans2, ans1);
            },
            "v" => {
                println!("Current state of ems:");
                println!("{:#?}", ems);
            }
            _ => (),
        }
        ans1 = String::new();
        ans2 = String::new();
    }
}

fn print_main_menu() {
    println!("HashMap and Vector Employee Management System");
    println!("                  Main Menu");
    println!("a - Add user to department");
    println!("v - View current status");
    println!("q - Quit");
}

fn get_answer(mut s: &mut String) {
    io::stdin().read_line(&mut s)
        .expect("Failed to read line");
    *s = s.trim().to_string();
}
