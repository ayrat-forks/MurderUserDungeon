use bevy::prelude::*;
use sysinfo::{ProcessorExt, System, SystemExt, UserExt};

use super::{ConsoleData, event::{EnteredConsoleCommandEvent, PrintConsoleEvent}};

pub fn commands_handler(
    mut cmd_reader: EventReader<EnteredConsoleCommandEvent>,
    mut console_writer: EventWriter<PrintConsoleEvent>,
    mut data: ResMut<ConsoleData>,
    mut sys: ResMut<System>,
) {
    for EnteredConsoleCommandEvent(cmd) in cmd_reader.iter() {
        // Don't do anything if the string is empty
        if cmd.is_empty() { return ; }

        let args: Vec<&str> = cmd.trim().split(' ').collect();

        if args[0] != "clear" {
            // first send what the user typed
            let mut user_input = String::from("> ");
            user_input.push_str(cmd.clone().trim());
            console_writer.send(PrintConsoleEvent(user_input));
        }

        match args[0] {
            "clear" => data.messages.clear(),
            "help" => console_writer.send(PrintConsoleEvent(display_help())),
            "motd" => console_writer.send(PrintConsoleEvent(print_motd(&mut sys, true))),

            _ => {
                console_writer.send(PrintConsoleEvent(format!("I didn't understand the command: \"{}\"", args[0])));
            }
        }
    }
}

fn display_help() -> String {
    let mut res = String::from("\nSHOWING AVAILABLE COMMANDS\n");

    let underline  = "==========================\n\n";
    res.push_str(underline);

    res.push_str("- help : Displays this message\n");
    res.push_str("- clear : Clears commands on the screen\n");
    res.push_str("- motd : Prints informations about YOUR computer\n");

    res
}

pub fn print_motd(sys: &mut System, should_refresh: bool) -> String {
    if should_refresh {
        sys.refresh_cpu();
        sys.refresh_memory();
        sys.refresh_system();
        sys.refresh_users_list();
    }
    
    let mut res = String::from("Welcome back on SafeOS 3.1\n");
    res.push_str("--------------------------\n");

    res.push_str(&format!("Username: {}\n\n", sys.users().last().unwrap().name()));

    res.push_str(&format!("System name:             {:?}\n", sys.name().unwrap()));
    res.push_str(&format!("System kernel version:   {:?}\n", sys.kernel_version().unwrap()));
    res.push_str(&format!("System OS version:       {:?}\n", sys.os_version().unwrap()));
    res.push_str(&format!("System host name:        {:?}\n\n", sys.host_name().unwrap()));

    res.push_str(&format!("Processors: {} at {:.2}GHz\n",
        sys.processors().len(), 
        sys.processors()[0].frequency() as f64 / 1000.0
    ));

    res.push_str(&format!("RAM: {} Gb\n", display_bar(
        60, 
        sys.used_memory() as f64 / 1000000.0, 
        sys.total_memory() as f64 / 1000000.0
    )));

    res
}

fn display_bar(width: usize, value: f64, total_value: f64) -> String {
    let percent = value / total_value;
    let nb_full_tiles = (percent * (width - 2) as f64) as usize;
    let rest_tiles = width - nb_full_tiles;

    let mut res = String::from("[");
    res.push_str(&String::from("=").repeat(nb_full_tiles));
    res.push_str(&String::from(" ").repeat(rest_tiles));
    res.push_str(&format!("] {:.2}/{:.2}", value, total_value));

    res
}