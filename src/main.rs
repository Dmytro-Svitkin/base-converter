fn war()->String{
    String::from("~WAR~")
}

fn main() {
    println!("{}",convert(&mut String::from("a"),&mut String::from("a"),&mut String::from("a")));
}

fn base(x:usize)->String{
    let base_62:&str="0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    if x>0&&x<63{return base_62[..x].to_string()};
    war()
}

fn to_base_10(x:&mut String,in_base:&mut String)->Result<usize,String>{
    let mut result:usize=0;
    let mut pwr:u32=0;
    let in_base_len:usize=in_base.len();
    for n in x.chars().rev(){
        let dig:usize=match in_base.chars().position(|c| c == n){
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

    war()
}