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
use super::write::add_date_to_log_file_name;
use super::trace::*;
use super::debug::*;
use super::info::*;
use super::warn::*;
use super::error::*;
use super::fatal::*;

#[test]
fn test_logging_of_all_levels ()
{
        let log_level: u8 = 0;
        
        let _ = String::from("_trace_").log_as_trace( &log_level, "test.log");
        let _ = String::from("_debug_").log_as_debug( &log_level, "test.log");
        let _ = String::from("_info_").log_as_info( &log_level, "test.log");
        let _ = String::from("_warn_").log_as_warn( &log_level, "test.log");
        let _ = String::from("_error_").log_as_error( &log_level, "test.log");
        let _ = String::from("_fatal_").log_as_fatal( &log_level, "test.log");
        
        let log_name = add_date_to_log_file_name( String::from("test.log") );
                
        let result_from_open = OpenOptions::new()
            .read(true)
            .open(log_name.as_str());        
            
        let mut file_content = String::new();
        
        match result_from_open
        {
                Ok(mut file) => 
                {
                        let _ = file.read_to_string( &mut file_content );
                },
                Err(error) => panic! ("{} occured when trying to open file {}",error, log_name.as_str()),
        }                 
        
        let file_lines = file_content.lines(); 
        let mut file_lines = file_lines.into_iter().rev();         
                
        let line_string = String::from 
        (
                match file_lines.next() 
                {
                        Some(string) => { string },
                        None => " ",
                }
        );
       assert! ( line_string.ends_with("FATAL: _fatal_") );
       
        let line_string = String::from 
        (
                match file_lines.next() 
                {
                        Some(string) => { string },
                        None => " ",
                }
        );
       assert! ( line_string.ends_with("ERROR: _error_") );
         
        let line_string = String::from 
        (
                match file_lines.next() 
                {
                        Some(string) => { string },
                        None => " ",
                }
        );
       assert! ( line_string.ends_with("WARN: _warn_") );
       
        let line_string = String::from 
        (
                match file_lines.next() 
                {
                        Some(string) => { string },
                        None => " ",
                }
        );
       assert! ( line_string.ends_with("INFO: _info_") );
       
        let line_string = String::from 
        (
                match file_lines.next() 
                {
                        Some(string) => { string },
                        None => " ",
                }
        );
       assert! ( line_string.ends_with("DEBUG: _debug_") );
       
        let line_string = String::from 
        (
                match file_lines.next() 
                {
                        Some(string) => { string },
                        None => " ",
                }
        );
       assert! ( line_string.ends_with("TRACE: _trace_") );
}
