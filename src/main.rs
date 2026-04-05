use std::{io::{self, Write}};

fn war()->String{
    String::from("~WAR~")
}

fn input(msg:&str)->String{
    print!("{}",msg);
    io::stdout().flush().unwrap();
    let mut result:String=String::new();
    io::stdin().read_line(&mut result).unwrap();
    result
}

fn main() {
    println!("Default bases must be between 1 and 62.\nTo use a custom base, you must input all characters/digits of that base.\nTo force a custom base, input two underscores first (e.g., __12345).\n");
    loop {
        let mut a:String=input("Value: ").trim().to_string();
        let mut b:String=input("Input base: ").trim().to_string();
        let mut c:String=input("Output base: ").trim().to_string();

        if b.chars().all(|ch|ch.is_ascii_digit()){
            let recol:usize=b.parse::<usize>().unwrap();
            if recol>62{println!("~WAR~ Bases cannot exceed 62, unless it is a custom base.")}
            else if recol<1{println!("~WAR~ Bases cannot be below 1.")}
            else{b=base(recol)}
        };
        if b.starts_with("__"){b=b[2..].to_string()};

        if c.chars().all(|ch|ch.is_ascii_digit()){
            let recol:usize=c.parse::<usize>().unwrap();
            if recol>62{println!("~WAR~ Bases cannot exceed 62, unless it is a custom base.")}
            else if recol<1{println!("~WAR~ Bases cannot be below 1.")}
            else{c=base(recol)}
        };
        if c.starts_with("__"){c=c[2..].to_string()};

        println!("{}\n",convert(&mut a,&mut b,&mut c))
    }
}

fn base(x:usize)->String{
    let base_62:&str="0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    if x>0&&x<63{return base_62[..x].to_string()};
    war()
}

fn to_base_10(x:&mut String,in_base:&mut String)->Result<usize,String>{
    let in_base_len:usize=in_base.len();
    if in_base_len==1{return Ok(x.len())};
    let mut result:usize=0;
    let mut pwr:u32=0;
    for n in x.chars().rev(){
        let dig:usize=match in_base.chars().position(|c|c==n){
            Some(i)=>{i},_=>{return Err(war())}
        };
        result+=dig*(in_base_len.pow(pwr));pwr+=1;
    };
    return Ok(result);
}

fn from_base_10(mut x:usize,out_base:&mut String)->String{
    let mut result:String=String::new();
    let out_base_len:usize=out_base.len();
    if x==0{return out_base.chars().nth(0).unwrap().to_string()}
    while x>0{
        result.push(out_base.chars().nth(x%out_base_len).unwrap());
        x/=out_base_len;
    }
    return result.chars().rev().collect();
}

fn convert(x:&mut String,in_base:&mut String,out_base:&mut String)->String{
    if in_base!=&mut war()&&out_base!=&mut war(){
        let local_result:Result<usize,String>=to_base_10(x, in_base);
        if local_result!=Err(war()){
            return from_base_10(local_result.unwrap(), out_base);
        }
    }
    war()
}