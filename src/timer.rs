use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use super::Result;
use team::Team;
use tmux;

pub fn run(time_per_driver_in_minutes: &f64, team: &mut Team) -> Result<()> {
    let time_per_driver_in_seconds = time_per_driver_in_minutes * 60.0;

    let mut elapsed_time = 0.0;

    loop {
        if is_time_for_next_driver(&time_per_driver_in_seconds, elapsed_time) {
            prompt_user(team)?;
        };

        println!("{}", team.driver);
        elapsed_time += 1.0;
        sleep(Duration::from_secs(1))
    }
}

fn prompt_user(team: &mut Team) -> Result<()> {
    let prompt_command = r#"
    echo 'Continue mobbing? [y/n]' && \
        read input && \
        echo $input > /tmp/mob
    "#;

    let exit_status = tmux::new_window_with_command(prompt_command)?;

    if exit_status.success() && is_continue()? {
        team.next_driver();
        Ok(())
    } else {
        exit(1);
    }
}

fn is_continue() -> Result<bool> {
    let mut file = File::open("/tmp/mob")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents.trim().to_lowercase() == "y")
}

fn is_time_for_next_driver(time_per_driver: &f64, elapsed_time: f64) -> bool {
    if elapsed_time != 0.0 && elapsed_time % time_per_driver == 0.0 {
        true
    } else {
        false
    }
}
