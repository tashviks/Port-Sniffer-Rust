use std::{env, net::IpAddr , str::FromStr};

struct Arguments{
    flag : String,
    ipadr : IpAddr,
    threads : u16,
}

impl Arguments {
    fn new(args : &[String]) -> Result<Arguments , &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many argumenst");
        }

        let f = args[1].clone();

        if let Ok(ipadr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipadr,
                threads: 4,
            });
        }
        else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && flag.len()==2 {
                println!(
                    "Usage: -j to select how many threads you want
                \n\r       -h or -help to show this help message"
                );
                return Err("Help");
            }
            else if flag.contains("h") || flag.contains("help"){
                return Err("too many argument");
            }
            else if flag.contains("-j"){
                let ipadr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                };

                let threads = match args[2].parse::<u16>(){
                    Ok(s)=> s,
                    Err(_) => return Err("Invalid Thread Number"),
                }
                return Ok(Arguments{
                    threads,
                    flag,
                    ipadr,
                });
            }
            else{
                    return Err("invalid syntax");
            }
            
        }



    }
}
fn main() {
    let args : Vec<String> = env::args().collect();
    let program = args[0].clone();

}
