pub mod level;
pub mod debug;
pub mod error;
pub mod fatal;
pub mod info;
pub mod trace;
pub mod warn;
pub mod write;
mod time;

#[cfg(test)]
mod test_set;

use fatal::LogAsFatal;

pub fn self_test( log_level: &u8, log_file_name: &str ) -> Result<String,String>
{
       match ("Test logging at start").log_as_fatal( log_level, log_file_name )
       {
                Ok(_) => 
                { 
                        let success_string = format!("Logging to {} is working", log_file_name);
                        success_string.log_as_fatal( log_level, log_file_name )
                },
                Err(error) => 
                { 
                        Err (format! ("! Logging not working! Error {} occured ! ", error) )
                },         
       }      
}       
