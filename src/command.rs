use super::*;
use utils::*;
use std::{error::Error, process::exit};
pub fn command_parse() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "start" => {
                start()
            },
            "node" => {
                if args.len() > 2 {
                    match args[2].as_str() {
                        "add" => {
                            if args.len()<4{
                                command_help_node_add();
                                return
                            }
                            match node_add(args[3].to_string()){
                                Ok(_) => {},
                                Err(err) => {
                                    ColorPrint::redln(format!("add node failed, err:{}",err.to_string()));
                                }
                            }
                        },
                        "del" => node_del(),
                        "list"|"ls" => node_ls(),
                        _|"-h"|"help" => command_help_node(),
                    }
                } else {
                    command_help_node();
                    return;
                }
            }
            "test" => cluster::cluster_add("127.0.0.1:8000").unwrap(),
            _|"-h"|"help" => command_help(),
        }
    } else {
        command_help();
    }
}

fn start() {
    super::global_state::STATE
        .lock()
        .unwrap()
        .insert("start_server", true);
}

fn node_add(address:String) -> Result<(), Box<dyn Error>>{
    let url = format!("http://{}/api/node/id", address);
    let id = reqwest::blocking::get(url)?.text()?;
    if id == utils::id_generator().unwrap(){
        ColorPrint::redln(format!("please ensure the address isn't local's address => {}",address));
        exit(0)
    }
    let cluster = cluster::cluster_json().lock().unwrap();
    match cluster.as_object().unwrap().get(&id){
        Some(_)=>{
            ColorPrint::redln(format!("this node already exists"));
            exit(0)
        },
        None => {
            drop(cluster);
            match cluster::cluster_add(address.as_str()){
                Ok(_) => {},
                Err(err) => {
                    ColorPrint::redln(format!("add node failed, err:{}",err.to_string()));
                    exit(0)
                }
            }
        }
    }
    ColorPrint::greenln(format!("success"));
    Ok(())
}
fn node_del() {}
fn node_ls() {}

fn command_help() {
    println!("--------------");
    println!("|    help    |");
    println!("--------------");
}
fn command_help_node() {
    println!("--------------");
    println!("| node help  |");
    println!("--------------");
}

fn command_help_node_add(){
    println!("-----------------");
    println!("| node add help  |");
    println!("-----------------");
}
