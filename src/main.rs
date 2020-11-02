extern crate rustls;
use structopt::StructOpt;
use std::io::{Read,Write};
use std::time;
use webpki;
use webpki_roots;
use std::io;
use std::net::{Ipv4Addr,ToSocketAddrs,TcpListener,TcpStream};
use regex::Regex;
use std::convert::TryFrom;
use rustls::Session;

#[derive(Debug,StructOpt)]
#[structopt(name = "Profiler System", about = "req")]
struct Cli{ 
    #[structopt(short="u",long ="url",help="The tool will make an HTTP request to the URL and print the response directly to the console",default_value ="www.google.com")] 
    url: String,
    #[structopt(short="p",long="profile",help="Number of times the request will be sent to the URL",default_value = "3")]
    profile: u8    
}
struct Response{
    http_status: String,
    time_elapsed: time::Duration,
    res_size:usize
}
fn send_request( url : &str)->Result<Response,Box<dyn std::error::Error>>{
    //This function send request to url and returns Response Struct 
    
    let full_addr = format!("{}:443",url);    
    let mut socket = TcpStream::connect(&full_addr).unwrap();
    let mut config = rustls::ClientConfig::new();
    config.root_store.add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    let arc = std::sync::Arc::new(config);
    let dns_name = webpki::DNSNameRef::try_from_ascii_str(url).unwrap();
    let mut client = rustls::ClientSession::new(&arc,dns_name);
    let mut stream = rustls::Stream::new(&mut client,&mut socket);
    let mut request_data = String::new();
    let req_body = format!("GET / HTTP/1.1\r\nHost: {}\r\nConnection:close\r\n\r\n",url).to_string();
    request_data.push_str(&req_body);        
    let request = stream.write_all(request_data.as_bytes()).unwrap();
    let mut now = time::SystemTime::now();
    let mut buf = vec![];
    let res = stream.read_to_end(&mut buf).unwrap();    
    let string_res = String::from_utf8_lossy(&buf);
    let status_re = Regex::new(r"HTTP/1.(0|1) \d{3}").unwrap();
    let _time = now.elapsed().unwrap();  
    let status_code = match status_re.find(&string_res){
        Some(i)=>&string_res[(i.end()-3)..i.end()],
        None=>"not there"
    };
    Ok(Response{http_status:status_code.to_string(),time_elapsed:_time,res_size:res.clone()})
}
fn mean(numbers: &Vec<u32>) -> u32 {
    let sum: u32 = numbers.iter().sum();
    sum / numbers.len() as u32               
}
fn median(numbers: &mut Vec<u32>) -> u32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mean(&vec![numbers[mid - 1], numbers[mid]]) as u32                             
    }
    else {
        numbers[mid]                                
    }            
}
fn main() {
    let cli = Cli::from_args();
    println!("{:?}",cli);
    let _url : String = cli.url.clone();
    let _profile : u8 = cli.profile.clone();
    let mut time_vec:Vec<u32> = Vec::new();
    let mut err_count : u8 = 0;
    let mut err_code: Vec<String> = Vec::new();
    let mut res_size_vec:Vec<usize> = Vec::new();
    
    for _ in 0.._profile{
        let curr_req_val = send_request(&_url).unwrap();
        time_vec.push(curr_req_val.time_elapsed.subsec_micros());
        res_size_vec.push(curr_req_val.res_size);     
        if curr_req_val.http_status != "200"{
            err_count += 1;
            err_code.push(curr_req_val.http_status);                      
        }
    }
    let avg:u32 = mean(&time_vec);
    let median : u32 =median(&mut time_vec);
    println!("Number of request : {:?}",&_profile);
    println!("Average Time : {:?}",time::Duration::from_micros(avg as u64));
    println!("Median Time : {:?}",time::Duration::from_micros(median as u64));
    println!("Longest Request Time : {:?}",time::Duration::from_micros(*time_vec.iter().max().unwrap() as u64));
    println!("Shortest Request Time : {:?}",time::Duration::from_micros(*time_vec.iter().min().unwrap() as u64));
    println!("Largest Reponse Size : {:?}",*res_size_vec.iter().max().unwrap());
    println!("Smallest Response Size : {:?}",*res_size_vec.iter().min().unwrap());
    println!("Errored Request : {}",err_count);
    println!("Error Codes : {:?}",err_code);
}
