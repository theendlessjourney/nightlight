use chrono::prelude::*;

fn main() {
    // let time_now = Local::now();
    // println!("time now is {}\n", time_now);
    // println!( "hour: {}, minute: {}\n", time_now.hour(), time_now.minute() );

    // let mut kill_cmd_line = std::process::Command::new( "pkill" );
    // kill_cmd_line.arg( "redshift" );

    // kill_cmd_line.spawn().expect( "kill cmd spawn failed." );

    // let pgrep_output = pgrep_cmd_line.output().expect( "pgrep failed" );
    // println!( "pgrep_output: {}", String::from_utf8_lossy( &pgrep_output.stdout ) );

    // let mut spawn_cmd_line = std::process::Command::new( "redshift" );
    // spawn_cmd_line.spawn().expect( "spawn cmd spawn failed." );

    let sleep_duration = std::time::Duration::from_secs( 60 * 3 );

    // loop {
    //     if let Ok(file_content) = std::fs::read_to_string( "/home/lhu/time.txt" ) {
    //         let file_content_trimmed = file_content.trim_end();
    //         if let Ok(number) = file_content_trimmed.parse::<u32>() {
    //             println!( "got {}", number );
    //         }
    //         else {
    //             println!("failed to parse '{}'", file_content_trimmed);
    //         }
    //     }
    //     else {
    //         println!("failed to read file")
    //     }

    //     std::thread::sleep(sleep_duration);
    // }

    // return;

    if let Ok(mut kill_child) = std::process::Command::new("pkill").arg("redshift").spawn() {
        kill_child.wait().expect("reset cmd failed");
    }

    loop {
        if let Some(child) = launch_nightlight(Local::now()) {
            loop {
                if time_check_at_night(Local::now()) {
                    std::thread::sleep(sleep_duration);
                } else {
                    kill_nightlight(child);
                    break;
                }
            }
        } else {
            std::thread::sleep(sleep_duration);
        }
    }
}

fn time_check_at_night(_time_now: DateTime<Local>) -> bool {
    let hour = _time_now.hour();
    // let hour = get_test_time_hour();

    let night_start = 19;
    let night_end = 7;

    if night_start <= hour {
        return true;
    }

    if hour < night_end {
        return true;
    }

    return false;
}

fn launch_nightlight(time_now: DateTime<Local>) -> Option<std::process::Child> {
    if !time_check_at_night(time_now) {
        return None;
    }

    match std::process::Command::new("redshift").spawn() {
        Ok(child) => return Some(child),
        _ => return None,
    }
}

fn kill_nightlight(mut proc: std::process::Child) {
    // kill
    proc.kill().expect("kill_nightlight kill faied");
    proc.wait().expect("kill_nightlight kill wait failed");

    // reset
    if let Ok(mut reset_child) = std::process::Command::new("redshift").arg("-x").spawn() {
        reset_child.wait().expect("reset cmd failed");
    }
}

// fn get_test_time_hour() -> u32 {
//     if let Ok(file_content) = std::fs::read_to_string( "/home/lhu/time.txt" ) {
//         if let Ok(number) = file_content.trim_end().parse::<u32>() {
//             // println!( "got {}", number );
//             return number;
//         }
//         else {
//             //println!("failed to parse '{}'", file_content_trimmed);
//         }
//     }
//     else {
//         //println!("failed to read file")
//     }
//     0
// }

// fn test() {
//     let mut spawn_cmd_line = std::process::Command::new("redshift");
//     if let Ok(mut child) = spawn_cmd_line.spawn() {
//         std::thread::sleep(std::time::Duration::from_secs(5));

//         // kill
//         child.kill().expect("child kill faied");
//         child.wait().expect("child kill wait failed");

//         // reset
//         if let Ok(mut reset_child) = std::process::Command::new("redshift").arg("-x").spawn() {
//             reset_child.wait().expect("reset cmd failed");
//         }
//     }
// }
