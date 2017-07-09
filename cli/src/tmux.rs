use errors::Result;
use std::process::{Command, ExitStatus};

// Open new window
// Run mob prompt
//      Write result to file
// Return input from user as stdout
// Parse input ->
//      if continue
//          set new driver
//      else
//          exit
//      end
pub fn new_window_with_command(command: &str) -> Result<ExitStatus> {
    Command::new("tmux")
        .arg("new-window")
        .arg(command)
        .status()
        .map_err(|error| error.into())
}
