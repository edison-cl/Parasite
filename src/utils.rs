use std::rc::Rc;


pub struct Config{
    pub port:String
}


pub fn parse_args()-> Config {
    let args: Vec<String> = std::env::args().collect();
    let args = Rc::new(args);
    let mut port = String::from("1000");
    for (index, item) in args.iter().enumerate() {
        let argss = Rc::clone(&args);
        match item.as_str() {
            "-p" => port = argss[index + 1].to_owned(),
            _ => {}
        };
    }
    Config{
        port:port
    }
}

