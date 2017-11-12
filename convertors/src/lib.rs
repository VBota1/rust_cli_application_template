pub mod str;
pub mod vec_string;

extern crate backend_declaration;
use backend_declaration::IsSupportedCommand;

extern crate command;
use command::Command;

#[cfg(test)]
mod test_set;

