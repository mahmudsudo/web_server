mod handler; 
mod server; 
mod router; 
use server::Server; 
fn main() {    // 
//Start a server  
      let server = Server::new("localhost:3000");    
      //Run zzzthe server    
      server.run(); }
