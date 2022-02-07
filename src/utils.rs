use std::{process::Command};
use regex::Regex;
use super::*;


pub struct Config{
    pub port:String
}

pub fn md5<S: Into<String>>(input: S) -> String {
    use crypto::digest::Digest;
    use crypto::md5::Md5;
    let mut md5 = Md5::new();
    md5.input_str(&input.into());
    md5.result_str()
}





pub fn id_generator() -> Option<String>{
    if cfg!(target_os = "windows"){
        let output = Command::new("ipconfig").arg("/all").output().expect("exec cmd failed");
        let text = String::from_utf8_lossy(&output.stdout).to_string();
        // 点心问号走天下
        let re = Regex::new(r"DUID  . . . . . . . : (.*?)\r\n").unwrap();
        let cap = re.captures(text.as_str()).unwrap();
        let uid = &cap[1];
        Some(md5(format!("{}{}",uid,config::global_data::port())))
    }else if cfg!(target_os = "linux"){
        let output = Command::new("sh").arg("-c").arg("demidecode -s system-serial-number").output().expect("exec cmd failed");
        let uid = String::from_utf8_lossy(&output.stdout).to_string();
        Some(md5(uid.as_str()))
    }else{
        None
    }
}


pub fn ip_get() -> Option<String> {
    use std::net::UdpSocket;
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}

pub struct ColorPrint{}
impl ColorPrint {
    pub fn redln(content:String){
        println!("\u{001B}[31;1m{}\u{001B}[0m",content);
    }

    pub fn greenln(content:String){
        println!("\u{001B}[32;1m{}\u{001B}[0m",content);
    }
}