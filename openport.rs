use std::net::TcpStream;
use std::env;
fn main() {
    let mut input = String::new();
    for argument in env::args(){
         input = argument;
    }
   if let Ok(_stream) = TcpStream::connect(input){
      println!("Porta Aberta!");

   }
}
