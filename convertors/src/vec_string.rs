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

use super::IsSupportedCommand;
use super::Command;

pub const NOTINITALIZED: &'static str = "Not initalized!";

pub trait ToCommands 
{
        fn to_commands (self) -> Vec<Command>;
}

impl ToCommands for Vec<String>
{       
        fn to_commands (self) -> Vec<Command>
        {                
                let mut all_commands: Vec<Command> = Vec::new();

                let mut is_command_available = false;
                let mut name: String = String::from(NOTINITALIZED);
                let mut arguments: Vec<String> = Vec::new();
            
                for string in self
                {
                        if string.is_supported_command()
                        {
                                if is_command_available
                                {
                                        let arguments = arguments.clone();
                                        all_commands.push( Command { name, arguments } );
                                }                        
                                name = string;
                                arguments.clear();
                                is_command_available = true;                        
                        }   
                        else
                        {
                                arguments.push(string);
                        }
                }
                
                if is_command_available
                {        
                        all_commands.push( Command { name, arguments } );        
                }       
                
                all_commands
        }
}

pub trait ToVecU8
{
        fn to_vec_u8 (self) -> Vec<u8>;
}

impl ToVecU8 for Vec<String>
{
        fn to_vec_u8 ( self ) -> Vec<u8>
        {      
                let string_vector: Vec<String> = self.into_iter().map( |a| a.to_string() ).collect();       
                let one_string = string_vector.as_slice().join(" ");
                one_string.into_bytes()      
         }
}
