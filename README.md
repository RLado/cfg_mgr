# cfg_mgr: A basic configuration manager for Rust
 
 This crate is built to import a plain text configuration file into a machine 
 readable structure.
 
 It's use is very simple, just call load(path :&str) on a properly formatted 
 config file and all it's data will be imported into a HashMap containing a 
 String key and a value in the form of the CfgData structure (defined within 
 this crate). The user will require prior knowledge whether to access the 
 numeric or string field. 
 
 By default all parseable numerical values will be parsed into the numeric 
 field as f64. Multiple numerical entries must be separated by ','. If any of 
 the values of a field can not be parsed the result is dumped in the string 
 field of CfgData.
 
 The proper format of a configuration file is as such:
 --------------------
 file: [config.cfg]
 --------------------
 
 ```
 # This is a comment
 foo = 3.1415
 bar = 1e-3 # comment
 foobar = 3.1415, 1e-3 # multiple arguments are allowed
 
 path = some/path/example.txt # this can't be parsed as f64 so it's a string
 ```
 --------------------
 
 
 ## Examples
 
 The following example loads a file named "config.cfg" and prints out it's 
 parsed contents.
 
 ```rust
 use cfg_mgr;

 fn main() {
     // Open a configuration file (ignoring errors)
     let config = cfg_mgr::load("config.cfg").unwrap();
 
     // Loop over all the keys of the configuration HashMap
     for key in config.keys() {
         print!("{}: ", key);
 
         // Print all numerical values (if any) for a particular key
         for i in 0..config.get(key).unwrap().numeric.len(){
             print!("{}, ", config.get(key).unwrap().numeric[i]);
         }
 
         // Print the string data of a key (if any) (separate using ;)
         println!(";{}", config.get(key).unwrap().string);
     }
 }
 ```