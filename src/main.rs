use structopt::StructOpt;
use std::io::{Read,Write};
use openssl::ssl::{SslMethod, SslConnectorBuilder};
use std::time::Duration;
use std::io;
use std::net::{Ipv4Addr,ToSocketAddrs,TcpListener,TcpStream};
#[derive(Debug,StructOpt)]
#[structopt(name = "Profiler System", about = "req")]
struct Cli{ 
    #[structopt(short="u",long ="url",help="The tool will make an HTTP request to the URL and print the response directly to the console",default_value ="google.com")] 
    url: String,
    #[structopt(short="p",long="profile",help="Number of times the request will be sent to the URL",default_value = "1")]
    profile: u8 
    
}
fn get( url : &str)->io::Result<()>{
    let connector = SslConnectorBuilder::new(SslMethod::tls()).unwrap().build();
    let full_addr = format!("{}:443",url);    
    let stream = TcpStream::connect(&full_addr).unwrap();
    let mut stream = connector.connect(&url,stream);
    print!("{:?}",stream);
    let mut request_data = String::new();
    let req_body = format!("GET / HTTP/1.1\r\nHost: {}\r\nConnection:close\r\n\r\n",url).to_string();
    request_data.push_str(&req_body);        
    println!("Request \n-------- \n{:?} \n--------", request_data);
    let request = stream.write_all(request_data.as_bytes())?;
    println!("request \n-------- \n {:?} \n--------", request);
    let mut buf = String::new();
    let result = stream.read_to_string(&mut buf)?;
    println!("result \n-------- \n {} \n--------", result);
    println!("{}", buf);
    Ok(())
}
fn main() {
    let cli = Cli::from_args();
    println!("{:?}",cli);
    let _url : String = cli.url.clone();
    let _profile : u8 = cli.profile.clone();
    get(&_url);        
}

    
    


