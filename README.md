The following Command Line Application profiles the server request time for any given url. To run the tool you can run the following command
        
    cargo build 
        or
    cargo run
Given that rust is already installed, you can find build folder within the directory after the command is ran.

The profiler can used the file within the using following commands
    
    >./http_profiler -u www.google.com -p 3
Here the params are

    -u | --url = The url to test (default = www.google.com)
    -p | --profile = Number of requests to be sent to the server(default= 3)

The output of the program will look something like this.

    Number of request : 3
    Average Time : 650.951ms
    Median Time : 637.889ms
    Longest Request Time : 734.43ms
    Shortest Request Time : 580.535ms
    Largest Reponse Size : 51339
    Smallest Response Size : 51284
    Errored Request : 0
    Error Codes : []