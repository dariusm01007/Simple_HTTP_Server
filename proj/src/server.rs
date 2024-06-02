use std::net::TcpListener;

// Struct called Server that contains an address (type = String literal)
pub struct Server{
    addr : String,
}

// In Rust, the implementation and declaration of a struct cannot be in the same block of code
// Typically in C++ people make .h & .cpp files anyways, so this isn't too different
impl Server{
    // Constructor
    // The convention for Rust is that a constructor is called "new"
    pub fn new (addr : String) -> Self{
        Self{
            addr
        }
    }

    // Methods
    // For a "once and done" application, this is fine. Ownership will be transferred to "run()", is self is to be reused, then 
    // Change it to : run(&mut self){}
    pub fn run(self){
        println!("Listening on {}",self.addr);

        // unwrap allows for a termination of the program if there is an error ""
        // Non Recoverable Error
        let listener = TcpListener::bind(&self.addr).unwrap(); 

        // Infinite loop
        // Can also just use "loop"
        while true{
            listener.accept();
        }
    }
}