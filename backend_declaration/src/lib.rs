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

#[cfg(test)]
mod test_set;

extern crate backend_command_example;
use backend_command_example::*;

extern crate command;
use command::Command;

extern crate about;
use about::*;

pub const HELP: &'static str = "help";

pub const SUPPORTEDCOMMANDS: &'static [&'static str] = &[ BACKENDCOMMANDEXAMPLE, ABOUT, HELP ];

pub fn run (to_execute: Command) -> Result<String,String>
{
        if to_execute.name.eq(BACKENDCOMMANDEXAMPLE)
        {        
                backend_command_example::main()
        }
        else  if to_execute.name.eq(ABOUT)
        {
                about::main()
        }       
        else  if to_execute.name.eq(HELP)
        {
                Ok (help())
        }                   
        else
        {
                Err ( format!( "Command {} is not supported", to_execute.name ) )     
        }                       
}

pub fn help () -> String
{
        let mut result: String = String::from 
        (
"Command: help\nUsage: help\n
"
        );
                
        result = result + about::help().as_str() + "\n";
        result = result + backend_command_example::help().as_str();
        result
}

pub trait IsSupportedCommand 
{
        fn is_supported_command(&self) -> bool;
}

impl IsSupportedCommand for String 
{
        fn is_supported_command (&self) -> bool 
        {         
                SUPPORTEDCOMMANDS.iter().any( |&scmd| scmd.eq(self) )                   
        }
}
