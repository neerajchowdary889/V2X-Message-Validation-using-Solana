#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
extern crate prettytable;

use std::io;
use std::collections::HashMap;
use prettytable::{Table, Row, Cell};

#[derive(Debug)]
struct Voting {
    Yes: i64,
    No: i64,
    Status: String,
}

fn main(){
    let vec = inputTovector();
    
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Index"), Cell::new("Item")]));
    for (index, item) in vec.iter().enumerate() {
        table.add_row(Row::new(vec![Cell::new(&index.to_string()), Cell::new(item)]));
    }
    table.printstd();

    println!("\n<-----------Voting Starts----------->");

    let map = voting(&vec);

    println!("<-----------Voting Ends----------->\n");

    for (key, value) in &map{
        println!("{:?}: {:?}", key, value);
    }
}

fn inputTovector() -> Vec<String> {
    let mut vec = Vec::new();

    println!("How many items you need to Input: ");
    let mut NumbOfItems = String::new();
    io::stdin().read_line(&mut NumbOfItems).expect("Failed to read line");
    let num: i32 = NumbOfItems.trim().parse().expect("Please type a number");

    let mut i: i32 = 0;
    while i < num {
        println!("Enter the item: {}/{}", i+1, num);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        vec.push(input.trim().to_string());
        i += 1;
    }
    vec
}

fn voting(vec: &Vec<String>) -> HashMap<String, Voting>{

    let mut map = HashMap::new();
    let len = vec.len() as i32;
    let mut j: i32 = 0;

    while j < len{
        let mut yes: i64 = 0;
        let mut no: i64 = 0;

        println!("Input the Item you want to vote or\nEnter ./!1 to stop voting\n>>Item: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim() == "./!1"{
            break;
        }

        let int = index(vec.clone(), input.trim().to_string());
    if int == -1{
        println!("{} is not in the list", input.trim());
        
        println!("\nEnter 1 to continue\nElse to stop voting: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if let Ok(choice) = input.trim().parse::<i64>() {
            if choice == 1 {
                println!(" ");
                continue;
            } else {
                break;
            }
        } else {
            break;
        }

    }else{
        let i: String = vec[int as usize].clone();
        println!("\n:: For item {}\nEnter 1 for yes\n0 for no\nanything else to stop voting:",i);
        loop{
            let mut input = String::new();
            println!(">>Vote: ");
            io::stdin().read_line(&mut input).expect("Failed to read line");

            if let Ok(choice) = input.trim().parse::<i64>() {
                if choice == 1 {
                    yes += 1;
                } else if choice == 0 {
                    no += 1;
                } else {
                    break;
                }
            } else{
                break;
            }
        }

        let Percentage: f64 = percentage(yes, no);

        println!("\n----->> Yes: {}%  No: {}%",Percentage, 100.0-Percentage);
        if Percentage > 70.0 {
            map.insert(i.clone(), Voting{Yes: yes, No: no, Status: "Selected".to_string()});
            println!("----->> {} is selected",i);
        } 
        else{
            map.insert(i.clone(), Voting{Yes: yes, No: no, Status: "Not Selected".to_string()});
            println!("----->> {} is not selected",i);
        }
        println!("\n<--------------------->\n");
        j = j+1;
    }
}
    map
}

fn percentage(yes: i64, no: i64) -> f64{
    let total = yes + no;
    let percentage = (yes as f64 / total as f64) * 100.0;
    percentage
}

fn index(vec: Vec<String>, input: String) -> i32{
    for i in 0..vec.len(){
        if vec[i] == input{
            let j = i as i32;
            return j;
        }
    }
    return -1 as i32;
}