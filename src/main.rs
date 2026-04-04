const WAR:&str="~WAR~";

fn main() {
    println!("{}",base(16));
    let e:Result<usize, String>=x_to_base_10(&mut String::from("1A"),&mut base(16));
    { println!("{:?}",e)};
}

fn base(x:usize)->String{
    let base_62:&str="0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    if x>0&&x<63{return base_62[..x].to_string()};
    WAR.to_string()
}

fn x_to_base_10(x:&mut String,in_base:&mut String)->Result<usize,String>{
    let mut result:usize=0;
    let mut pwr:u32=0;
    let in_base_len:usize=in_base.len();
    for n in x.chars().rev(){
        let dig:usize=match in_base.chars().position(|c| c == n){
            Some(i)=>{i},_=>{return Err(WAR.to_string())}
        };
        result+=dig*(in_base_len.pow(pwr));pwr+=1;
    };
    return Ok(result);
}