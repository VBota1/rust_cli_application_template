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

pub const ABOUT: &'static str = "about";

pub fn main () -> Result<String, String>
{        
        let name_and_version: NameAndVersion = match get_name_and_version()
        {
                Ok(data) => { data },
                Err(error) => { return Err(error); },
        };
        
        Ok ( format! ("{} Version: {}\n{}", name_and_version.name, name_and_version.version, get_all_licences() ) )
}

pub fn help () -> String
{
        String::from (
"Command: about\nUsage: about
"
        )
}
 
struct NameAndVersion
{
        pub name: String,
        pub version: String,
}

fn get_name_and_version () -> Result<NameAndVersion, String>
{
        let file_name = "./Cargo.toml";
        let result_from_open = OpenOptions::new()
            .read(true)
            .open(file_name);   
            
        let mut file_content = String::new();            
            
        match result_from_open
        {
                Ok(mut file) => 
                {
                        let _ = file.read_to_string( &mut file_content );
                },
                Err(error) => 
                {
                        let error_string = format! ("{} occured when trying to open file {}",error, file_name);
                        return Err (error_string);
                }
        }     
        
        let file_lines = file_content.lines(); 
        let mut file_lines = file_lines.into_iter();           

        file_lines.next();
        
        let name = match get_last_string_between_quoutes ( file_lines.next(), file_name )
        {
                Ok(string) => { string },
                Err(error) => { return Err(error) },
        };
        
        let version = match get_last_string_between_quoutes ( file_lines.next(), file_name )
        {
                Ok(string) => { string },
                Err(error) => { return Err(error) },
        };        
        
        Ok ( NameAndVersion{name, version} )
}

fn get_last_string_between_quoutes (file_line: Option<&str>, file_name: &str ) -> Result<String, String>
{
        let line = String::from 
        (
                match file_line
                {
                        Some(string) => { string },
                        None => " ",
                }
        );
        
        let line_as_vector: Vec<_> = line.split('"').collect();
        
        let mut line_as_iterator = line_as_vector.into_iter().rev();
        
        line_as_iterator.next();
        
        match line_as_iterator.next()
        {
                Some(string) => { return Ok( String::from(string) ) },
                None => 
                {
                        let error_string = format! ("Error finding \" in line {} from file {}", line, file_name );
                        return Err (error_string);                        
                },
        }        
}

fn get_all_licences () -> String
{        
        format! ("Licence: \n{}\n3'rd party licences:\n{}", get_own_licence (), get_3rd_party_licence () )
}

fn get_own_licence () -> String
{
                String::from (
"\tCopyright (c) 2017 Viorel Bota\n
\tPermission is hereby granted, free of charge, to any person obtaining a copy\n\tof this software and associated documentation files (the \"Software\"), to deal\n\tin the Software without restriction, including without limitation the rights\n\tto use, copy, modify, merge, publish, distribute, sublicense, and/or sell\n\tcopies of the Software, and to permit persons to whom the Software is\n\tfurnished to do so, subject to the following conditions:\n
\tThe above copyright notice and this permission notice shall be included in all\n\tcopies or substantial portions of the Software.\n
\tTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\n\tIMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\n\tFITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\n\tAUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\n\tLIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\n
\tOUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE\n\tSOFTWARE.
"
                )
}

fn get_3rd_party_licence () -> String
{
                String::from (
"\tfor chronotope/chrono used in time.rs:\n\tCopyright (c) 2014, Kang Seonghoon.\n
\tPermission is hereby granted, free of charge, to any person obtaining a copy\n\tof this software and associated documentation files (the \"Software\"), to deal\n\tin the Software without restriction, including without limitation the rights\n\tto use, copy, modify, merge, publish, distribute, sublicense, and/or sell\n\tcopies of the Software, and to permit persons to whom the Software is\n\tfurnished to do so, subject to the following conditions:\n
\tThe above copyright notice and this permission notice shall be included in\n\tall copies or substantial portions of the Software.\n
\tTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\n\tIMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,\n\tFITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE\n\tAUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER\n\tLIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,\n
\tOUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN\n\tTHE SOFTWARE.
"
                )
}
