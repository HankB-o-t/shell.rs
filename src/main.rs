use std::io::{stdin,stdout,Write};use std::process::Command;/////////////
use std::env;use std::path::Path; fn read(input:&mut String)/////////////
{let _=stdout().flush();stdin().read_line(input).unwrap();}fn //shell.rs/
main(){loop{print!("$ ");let mut i=String::new();read(&mut i);///////////
let mut p=i.trim().split_whitespace();let cm=p.next().unwrap()//2024/////
;let a=p;if cm=="exit"{break;}match cm{"cd"=>{let nd=a.//////////////////
peekable().peek().map_or("/",|x|*x);let r=Path::new(nd);////HankB-o-t////
if let Err(e)=env::set_current_dir(&r){eprintln!("{}",e);////////////////
}},cm=>{let mut c=Command::new(cm).args(a).spawn().unwrap();c.wait();}}}}
