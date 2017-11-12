/*
MIT License

Copyright (c) 2017 Viorel Bota

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::fs::OpenOptions;
use std::io::prelude::*;
use super::time::*;

fn to_log (text_to_write: String, log_file_name: String ) -> Result<String, String>
{        
        let log_name = add_date_to_log_file_name(log_file_name);
        
        let result_from_open = OpenOptions::new()
            .append(true)
            .create(true)
            .open(log_name.as_str());
            
        let text_to_write = text_to_write.as_str();
        
        match result_from_open
        {
                Ok(mut file) =>  
                {
                        match file.write_all ( text_to_write.as_bytes() )
                        {
                                Ok(_) => 
                                {
                                        return Ok ( String::from("text was written to log file") )
                                },
                                Err(error) => 
                                {
                                        let error_string = String::from ( format! ("{}. Occured when trying to write to file {}", error, log_name.as_str()) );
                                        return Err  (error_string);
                                },
                        }
                },
                Err(error) => 
                {
                        let error_string = String::from ( format! ("{}. Occured when trying to open file {}",error, log_name.as_str()) );
                        return Err (error_string);
                },
        }        
}

pub trait LogAs
{
        fn log_as ( &self, log_level_text: &str, log_file_name: String ) -> Result<String, String>;
}

impl LogAs for String
{
        fn log_as ( &self, log_level_text: &str, log_file_name: String  ) -> Result<String, String>
        {
                        let timestamp: String = now_as_string_date_and_time();
                        let text_to_write: String = String::from("\n") + timestamp.as_str() + " " + log_level_text + ": " + self.as_str();
                        to_log( text_to_write, log_file_name )
        }
}

pub fn add_date_to_log_file_name ( log_file_name: String ) -> String
{
        let mut log_file_name = log_file_name;
        let insert_position = log_file_name.len() - 4;
        let date: String = String::from(" ") + now_as_string_date().as_str();
        log_file_name.insert_str( insert_position, date.as_str() );
        log_file_name
}
