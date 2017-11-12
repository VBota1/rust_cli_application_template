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

use std::env::args;
use super::vec_string::*;
use super::str::ToStringVec;

#[test]
fn vec_string_to_commands_no_arguments() 
{
        let arguments: Vec<String> = args().collect();
        let all_commands = arguments.to_commands();
        assert_eq!( all_commands.len(), 0 );
}

#[test]
fn vec_string_to_commands_no_commands_in_arguments() 
{
        let arguments: Vec<String> = "ana are mere".to_string_vec();
        let all_commands = arguments.to_commands();
        assert_eq!( all_commands.len(), 0 );
}

#[test]
fn vec_string_to_commands_one_command_as_argument() 
{
        let arguments: Vec<String> = "BC".to_string_vec();
        let all_commands = arguments.to_commands();
        assert_eq!( all_commands[0].name , "BC".to_string() );
}

#[test]
fn vec_string_to_commands_valid_arguments() 
{
        let arguments: Vec<String> = "    BC BC ana are mere BC  BC   BC    logg debugg bc small case".to_string_vec();
        let all_commands = arguments.to_commands ( );
        assert_eq!( all_commands.len(), 5 );
        let mut command_index = 0;
        assert_eq!( all_commands[command_index].name , "BC".to_string() ); 
        assert_eq!( all_commands[command_index].arguments.len() , 0 );
        command_index += 1;
        assert_eq!( all_commands[command_index].name , "BC".to_string() ); 
        assert_eq!( all_commands[command_index].arguments.len() , 3 );
        let mut argument_count = 0;      
        assert_eq!( all_commands[command_index].arguments[argument_count] , "ana".to_string() );        
        argument_count += 1;
        assert_eq!( all_commands[command_index].arguments[argument_count] , "are".to_string() );        
        argument_count += 1;        
        assert_eq!( all_commands[command_index].arguments[argument_count] , "mere".to_string() );                        
        command_index += 1;
        assert_eq!( all_commands[command_index].name , "BC".to_string() ); 
        assert_eq!( all_commands[command_index].arguments.len() , 0 );        
        command_index += 1;
        assert_eq!( all_commands[command_index].name , "BC".to_string() ); 
        assert_eq!( all_commands[command_index].arguments.len() , 0 );           
        command_index += 1;
        assert_eq!( all_commands[command_index].name , "BC".to_string() ); 
        assert_eq!( all_commands[command_index].arguments.len() , 5 );           
        argument_count = 0;      
        assert_eq!( all_commands[command_index].arguments[argument_count] , "logg".to_string() );        
        argument_count += 1;
        assert_eq!( all_commands[command_index].arguments[argument_count] , "debugg".to_string() );          
        argument_count += 1;
        assert_eq!( all_commands[command_index].arguments[argument_count] , "bc".to_string() );                   
        argument_count += 1;
        assert_eq!( all_commands[command_index].arguments[argument_count] , "small".to_string() );             
        argument_count += 1;
        assert_eq!( all_commands[command_index].arguments[argument_count] , "case".to_string() );             
}

#[test]
fn empty_vec_string_to_vec_u8 ()
{
        let arguments: Vec<String> = "".to_string_vec();
        let vec_u8 = arguments.to_vec_u8();
        assert_eq!( vec_u8.capacity(), 0 );
        
        let arguments: Vec<String> = "   ".to_string_vec();
        let vec_u8 = arguments.to_vec_u8();
        assert_eq!( vec_u8.capacity(), 0 );        
}


#[test]
fn vec_with_valid_string_to_vec_u8 ()
{
        let mut b = [0; 1];
        let character = '!'.encode_utf8(&mut b);
        let arguments: Vec<String> = character.to_string_vec();
        let vec_u8 = arguments.to_vec_u8();
        assert_eq!( vec_u8.capacity(), 1 );        
        assert_eq!( format!("{:?}",vec_u8), "[33]" );    
        
        let mut b = [0; 1];
        let character = '~'.encode_utf8(&mut b);
        let arguments: Vec<String> = character.to_string_vec();
        let vec_u8 = arguments.to_vec_u8();
        assert_eq!( vec_u8.capacity(), 1 );        
        assert_eq!( format!("{:?}",vec_u8), "[126]" );    
}

#[test]
fn str_to_vec_string ()
{
        let vec_string = "ana nu mai are mere".to_string_vec();
        assert_eq!( vec_string, ["ana","nu","mai","are","mere"] );            
}

#[test]
fn empty_str_to_vec_string ()
{
        let vec_string = "".to_string_vec();
        assert_eq!( vec_string.capacity(), 0 );            
}

