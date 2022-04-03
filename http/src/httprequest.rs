#[derive(Debug, PartialEq)] 
pub enum Method {  
      Get, Post, Uninitialized, }
      impl From<&str> for Method {    
          fn from(s: &str) -> Method { 
               match s {
                "GET" => Method::Get, 
               "POST" => Method::Post, 
                _ => Method::Uninitialized,}}}
                #[cfg(test)] 
                mod tests {
                    use super::*;
                    #[test]
                    fn test_version_into(){let m: Version = "HTTP/1.1".into();assert_eq!(m, Version::V1_1); }

                    #[test]
                    fn test_method_into() {
                        let m: Method = "GET".into(); assert_eq!(m, Method::Get);
                  }
                }
  #[derive(Debug, PartialEq)] 
  pub enum Version {    V1_1,    V2_0,    Uninitialized, }
  impl From<&str> for Version {
       fn from(s: &str) -> Version {
           match s {
               "HTTP/1.1" => Version::V1_1,
               "HTTP/2.0" => Version::V2_0,
                _ => Version::Uninitialized,        }    } }
  use std::collections::HashMap;
 #[derive(Debug, PartialEq)]
 pub enum Resource {
     Path(String), }
#[derive(Debug)] 
pub struct HttpRequest {
     pub method: Method,
      pub version: Version,
      pub resource: Resource,
       pub headers: HashMap<String, String>,
       pub msg_body: String,
     }
       impl From<String> for HttpRequest {    
           fn from(req: String) -> HttpRequest {
let mut parsed_version = Version::V1_1;
 let mut parsed_resource = Resource::Path("".to_string());
 let mut parsed_headers = HashMap::new();
let mut parsed_msg_body = "";
let mut parsed_method  = Method::Uninitialized;
       // Read each line in incoming HTTP request      
         for line in req.lines() {            
             // If the line read is request line, call function process_req_li        
            if line.contains("HTTP") { 
                 let (method, resource, version) = process_req_line(line); 
                 parsed_method = method;
                 parsed_version = version;
                  parsed_resource = resource;            
                // If the line read is header line, call function process_header_line()           
             } else if line.contains(":") {  
                  let (key, value) = process_header_line(line);   
                        parsed_headers.insert(key, value);  
                    
                          //  If it is blank line, do nothing          
                 } else if line.len() == 0 { 
                          // If none of these, treat it as message body            
                } 
                 else { 
                      parsed_msg_body = line;            
                    }       
                 }        
                    // Parse the incoming HTTP request into HttpRequest struct        
                    HttpRequest {
                        method: parsed_method, 
                         version: parsed_version, 
                          resource: parsed_resource,
                           headers: parsed_headers,
                           msg_body: parsed_msg_body.to_string(),
                 }
                }
            }
           fn process_header_line(s: &str) -> (String, String) {    // Parse the headerline into words split by separator (':')    
           let mut header_items = s.split(":");    
           let mut key = String::from("");    
           let mut value = String::from("");    // Extract the key part of the header    
           if let Some(k) = header_items.next() {
               key = k.to_string();   
             }    // Extract the value part of the header    
           if let Some(v) = header_items.next() {
               value = v.to_string() }
             (key, value) }

            fn process_req_line(s: &str) -> (Method, Resource, Version) {    // Parse the request line into individual chunks split by whitespaces.    
                let mut words = s.split_whitespace();    // Extract the HTTP method from first part of the request line    
                let method = words.next().unwrap();    // Extract the resource (URI/URL) from second part of the request line    
                let resource = words.next().unwrap();    // Extract the HTTP version from third part of the request line   
                 let version = words.next().unwrap();
                ( method.into(), Resource::Path(resource.to_string()), version.into() ) 
            }
            