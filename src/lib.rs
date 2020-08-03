use std::collections::HashMap;
use std::any::Any;
use std::fmt::{Debug};
use std::ops::Deref;

// #[derive(PartialEq)]
// #[derive(Clone)]
// pub enum ArgumanTypes{ //External types
//     U32, U64, I32, I64, F32, F64, STRING, BOOL, NONE
// }
//
// enum ArgumanValues{ //Internal valued types
//     U32(u32), U64(u64), I32(i32), I64(i64), F32(f32), F64(f64), STRING(String), BOOL(bool), NONE
// }
#[derive(Debug)]
pub enum MagicValue{
    VALUE(Box<dyn Any>), NONE
}

impl MagicValue {
    pub fn opt_unwrap<T: 'static>(&self) -> Option<&T>{
        if let MagicValue::VALUE(val) = self{

            let magic = val.deref();
            let val = magic.downcast_ref::<T>();
            return val;

        }
        None
    }

    pub fn unwrap<T: 'static>(&self) -> &T{
        let farquad = self;
        let value = farquad.opt_unwrap::<T>();
        let value = match value {
            Some(x) => x,
            None => panic!("Unwrapping unsuccessful."),
        };
        value
    }
}
/// An enum that contains the types of errors possible when parsing user arguments.
/// Each error type contains a String that is the error in question.
pub enum MagicErr{
    FlagErr(String),
    ParseErr(String),
    ValueErr(String),
    InputErr,
}


pub struct MagicArgman{
    raw_args:Vec<String>,
    value_map:HashMap<String, MagicValue>,
    error: Vec<MagicErr>,
}

impl MagicArgman{
    pub fn new(args:Vec<String>) -> Self{
        let args = args.as_slice()[1..].to_vec().clone();
        MagicArgman {raw_args: args, value_map: HashMap::new(), error: Vec::new()}
    }

    ///Adds an optional flag with a given datatype for user-input.
    pub fn flag<T: std::str::FromStr + 'static>(&mut self, flag:&str) -> &mut Self{

        self.flag_setter::<T>(flag);
        self
    }

    ///Internally used for flag methods.
    fn flag_setter<T: std::str::FromStr + 'static>(&mut self, flag:&str){
        let args = self.raw_args.clone();
        let formatted_flag = {
            let mut format = String::new();
            if flag.len() > 1{
                format = format!("--{}", flag) //usually, flags with more that one character require two hyphens
            } else{
                format = format!("-{}", flag) //one-character flag format
            }
            format
        };
        if args.contains(&formatted_flag){
            let index = args.iter().position(|r| r.eq(&formatted_flag)); //gets the index we want
            let index = index.unwrap();
            //println!("INDEX OF {}: {}", &formatted_flag, index);
            //functional stuff is nice.
            if index + 1 < args.len(){
                //inserts possible value
                let value:Result<T, T::Err> = args[index + 1].parse::<T>();
                if let Ok(value) = value{
                    self.value_map.insert(String::from(flag), MagicValue::VALUE(Box::new(value)));
                } else{
                    self.error.push(MagicErr::ParseErr(format!("{}", flag))); //pushes a parse error that indicated that wrong type was given.
                }
            } else{
                self.error.push(MagicErr::ValueErr(format!("{}", flag))); //pushes a value error that indicated that no value after flag was given.
            }
        }
    }

    /// Adds a required (non-optional) flag with a given datatype for user-input.
    /// If an error occurs and the flag is not detected, the error will be saved.
    /// Error can be checked by using the get_error() method.
    pub fn flag_req<T: std::str::FromStr + 'static>(&mut self, flag:&str) -> &mut Self{
        self.flag_setter::<T>(flag);
        if let None = self.value_map.get(flag){ //if there is NO value gathered from the user
            self.error.push(MagicErr::FlagErr(format!("{}", flag)))
        }
        self
    }

    ///Adds a flag that does not need further user input. Creates a new entry in HashMap with bool value true.
    pub fn flag_solo(&mut self, flag: &str) -> &mut Self{
        let args = self.raw_args.clone();
        let formatted_flag = format!("-{}", flag);
        if args.contains(&formatted_flag){
            self.value_map.insert(String::from(flag), MagicValue::VALUE(Box::new(true)));
        }
        self
    }

    ///Returns the possible value given for a flag as an Option<&T>
    pub fn get<T: 'static>(&self, key:&str) -> Option<&T>{
        let value = self.value_map.get(key);
        if let Some(value) = value{
            return value.opt_unwrap::<T>();
        }
        None
    }

    ///Returns all errors that occured while parsing the user's arguments
    pub fn get_errors(&self) -> &Vec<MagicErr>{
        &self.error
    }

    ///Returns true or false if an error occured while getting user input.
    pub fn error(&self) -> bool{
        if self.error.len() != 0{
            return true;
        }
        false
    }

    ///Adds an input field to the beginning of the application's arguments. Takes it as String. If no value is detected, it will panic.
    ///If used, input MUST be the first method to be called before adding any flags for it to be properly used.
    ///If no input is found chaos will ensue.
    pub fn input(&mut self) -> &mut Self{
        let args = &self.raw_args;
        let input = args.first();
        if let Some(input) = input{
            self.value_map.insert(String::from("___RESERVED_INPUT___"), MagicValue::VALUE(Box::new(input.clone())));
        } else{
            self.error.push(MagicErr::InputErr);
        }
        self
    }

    pub fn get_input(&self) -> Option<&String> {
        let value = self.value_map.get("___RESERVED_INPUT___");
        if let Some(value) = value{
            return value.opt_unwrap::<String>();
        }
        None
    }



}