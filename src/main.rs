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

use std::env;

extern crate convertors;
use convertors::vec_string::ToCommands;

extern crate backend_declaration;

extern crate log;
use log::trace::LogAsTrace;
use log::error::LogAsError;

#[cfg( not(any( debug_log_level, info_log_level, warn_log_level, error_log_level, fatal_log_level)) )]
pub const LOGLEVEL: u8 = 0;
#[cfg(debug_log_level)]
pub const LOGLEVEL: u8 =1;
#[cfg(info_log_level)]
pub const LOGLEVEL: u8 =2;
#[cfg(warn_log_level)]
pub const LOGLEVEL: u8 =3;
#[cfg(error_log_level)]
pub const LOGLEVEL: u8 =4; 
#[cfg(fatal_log_level)]
pub const LOGLEVEL: u8 =5;

pub const LOGFILENAME: &'static str = "command_handler.log";

fn main() 
{                                 
        if let Err(error) = log::self_test( &LOGLEVEL, &LOGFILENAME )
        { 
                println! ("! Logging not working! Error {} occured ! ", error); 
        }
              
        let args: Vec<String> = env::args().collect();
        let _ = "Main: Convert CLI arguments to commands".log_as_trace( &LOGLEVEL, &LOGFILENAME );        
        let all_commands =  args.to_commands();
        
        for cmd in all_commands
        {                
                let _ = (format!("Main: Execute {} {:?}", cmd.name, cmd.arguments )).log_as_trace( &LOGLEVEL, &LOGFILENAME ); 
                match backend_declaration::run (cmd)
                {
                        Ok( success ) => 
                        { 
                                if !success.is_empty()
                                {
                                        println! ("{}",success);
                                }
                                let _ = ("Command was executed with success").log_as_trace( &LOGLEVEL, &LOGFILENAME );  
                        },
                        Err( error ) => 
                        {
                                let _ = error.log_as_error( &LOGLEVEL, &LOGFILENAME ); 
                                println! ("{}",error);
                        } 
                }
        }
}

