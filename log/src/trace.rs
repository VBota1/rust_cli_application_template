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

use super::level::TRACE;
use super::level::get_log_level_error_string;
use super::write::LogAs;

pub trait LogAsTrace
{
        fn log_as_trace ( &self, log_level: &u8, log_file_name: &str ) -> Result<String, String>;
}

impl LogAsTrace for String
{
        fn log_as_trace ( &self, log_level: &u8, log_file_name: &str ) -> Result<String, String>
        {

                if log_level.gt(&TRACE)
                {
                        return Err ( get_log_level_error_string ( &TRACE, log_level ) );
                }
                
                (&self).log_as ("TRACE" , String::from(log_file_name) )
      
        }
}

impl LogAsTrace for str
{
        fn log_as_trace ( &self, log_level: &u8, log_file_name: &str ) -> Result<String, String>
        {
                let text_to_log = String::from(self);
                
                if log_level.gt(&TRACE)
                {
                        return Err ( get_log_level_error_string ( &TRACE, log_level ) );
                }
                
                (text_to_log).log_as ("TRACE" , String::from(log_file_name) )
      
        }
}
