use std::collections::HashMap;
use std::fs;
use std::io::Error;

#[derive(Debug)]
struct Parser{
    data: HashMap<String, HashMap<String, String>>
}

impl Parser {
    pub fn new() -> Parser{
        return Parser{data: HashMap::new()};
    }

    fn add_section(&mut self, section: &String) -> Result<(), &'static str>{
        validate(section)?;
        self.data.insert(String::clone(section), HashMap::new());
        Ok(())
    }

    fn add_key_val(&mut self, key: String, val: String, candidate_section: Option<String>) -> Result<(), &'static str>{
        let section : String;
        match candidate_section{
            None => {
                section = String::from("default");
            },
            Some(sec) => {
                section = sec;
            }
        }
        validate(&section)?;
        validate(&key)?;
        validate(&val)?;

        match self.data.get_mut(&section) {
            Some(map) => {
                map.insert(key, val);
            },
            None => {
                self.add_section(&section);
                self.add_key_val(key, val, Some(section));
            }
        }
        Ok(())
    }

    fn read_from_string(&mut self, str: String) -> Result<(), &'static str>{
        let mut section = "default";
        for line in str.lines(){
            let line = line.trim();
            if is_section(line){
                section = line.trim_start_matches("[");
                section = section.trim_end_matches("]");
                match self.add_section(&section.to_string()) {
                    Err(err) => return Err(err),
                    _ => {},
                }
            }else if is_key_val(line){
                let pair = line.split("=").collect::<Vec<&str>>();
                let key = pair[0];
                let val = pair[1];
                match self.add_key_val(key.to_string(), val.to_string(), Some(section.to_string())) {
                    Err(err) => return Err(err),
                    _ => {},
                }
            }else if !line.is_empty(){
                return Err("invalid syntax");
            }
        }
        Ok(())
    }


    fn read_from_file(&mut self, filename: &str) -> Result<(), Error>{
        let contents = fs::read_to_string(filename)?;
        match self.read_from_string(contents){
            Err(err) => return Err(Error::new(std::io::ErrorKind::Other, err)),
            _ => {},
        }
        Ok(())
    }
}

fn is_key_val(line: &str) -> bool{
    return line.matches("=").count() == 1;
}

fn is_section(line: &str) -> bool{
    return line.starts_with("[") && line.ends_with("]");
}

fn validate(str: &String) -> Result<(), &'static str> {
    if str.is_empty() || str.contains(" ") || str.contains(";") {
        return Err("not valid");
    }
    return Ok(());
    
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test_add_section(){
        let mut parser = Parser::new();
        assert_eq!(parser.add_section(&String::from("sec1")), Ok(()));
        assert_eq!(parser.data.contains_key("sec1"), true);
        assert_eq!(parser.add_section(&String::from("s s")), Err("not valid"));
    }
    #[test]
    fn test_add_key_val(){
        let mut parser = Parser::new();
        //valid test
        let res = parser.add_key_val(String::from("key1"), String::from("val1"), Some(String::from("default")));
        assert_eq!(res, Ok(()));
        assert_eq!(parser.data.contains_key("default"), true);
        assert_eq!(parser.data.get("default").unwrap().contains_key("key1"), true);
        assert_eq!(parser.data.get("default").unwrap().get(&String::from("key1")).unwrap(), &String::from("val1"));

        //invalid test
        assert_eq!(parser.add_key_val(String::from("k k"), String::from("val1"), Some(String::from("default"))), Err("not valid"));
    }

    #[test]
    fn test_validate(){
        assert_eq!(validate(&String::from("s s")), Err("not valid"));
        assert_eq!(validate(&String::from("")), Err("not valid"));
        assert_eq!(validate(&String::from("s;s")), Err("not valid"));
        assert_eq!(validate(&String::from("s")), Ok(()));
    }

    #[test]
    fn test_read_from_string(){
        let mut parser = Parser::new();
        //valid
        let res = parser.read_from_string(String::from("[sec1]\nkey1=val1"));
        assert_eq!(res, Ok(()));
        assert_eq!(parser.data.contains_key("sec1"), true);
        assert_eq!(parser.data.get("sec1").unwrap().contains_key("key1"), true);
        assert_eq!(parser.data.get("sec1").unwrap().get(&String::from("key1")).unwrap(), &String::from("val1"));

        //invalid
        let res = parser.read_from_string(String::from("[sec1]\nss\n"));
        assert_eq!(res, Err("invalid syntax"));
    }

    #[test]
    fn test_read_from_file(){
        let mut parser = Parser::new();
        let res = parser.read_from_file("read.txt");
        // assert_eq!(res, Ok(()));
        assert_eq!(parser.data.contains_key("sec1"), true);
        assert_eq!(parser.data.get("sec1").unwrap().contains_key("key1"), true);
        assert_eq!(parser.data.get("sec1").unwrap().get(&String::from("key1")).unwrap(), &String::from("val1"));
    }
}
