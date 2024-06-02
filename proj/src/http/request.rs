// method (enum) is not in the request module; however, both the method module and request module are in the http module... 
// "Super" is referring to the parent module (http)
// module name :: enum
use super::method::Method; 

// Creating a struct (public) with path, query_string, and method as its parameters
pub struct Request {
    path         : String,
    query_string : Option <String>,
    method       : Method,
}
