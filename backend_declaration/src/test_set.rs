use super::*;

#[test]
fn recognize_valid_commands() 
{
        for cmd in SUPPORTEDCOMMANDS
        {
                assert! (cmd.to_string().is_supported_command());
        }
}

#[test]
fn recognize_invalid_commands() 
{
        for cmd in SUPPORTEDCOMMANDS
        {
                let updated_cmd = cmd.to_string();
                let updated_cmd = updated_cmd + " ";                
                assert! ( !updated_cmd.is_supported_command() );
        }
        
        for cmd in SUPPORTEDCOMMANDS
        {
                let updated_cmd = " ".to_string();
                let updated_cmd = updated_cmd + cmd;                
                assert! ( !updated_cmd.is_supported_command() );
        }        
        
        for cmd in SUPPORTEDCOMMANDS
        {
                let mut updated_cmd = cmd.to_string();      
                updated_cmd.pop();
                
                assert! ( ! updated_cmd.is_supported_command() );
        }        
        
        for cmd in SUPPORTEDCOMMANDS
        {
                let mut updated_cmd = cmd.to_string();      
                updated_cmd.remove(0);
                
                assert! ( ! updated_cmd.is_supported_command() );
        }               
}

