pub mod app;
pub mod handler;
pub mod output;

use clap::{Arg, Command, ArgMatches};
use clap::{crate_name, crate_version, crate_description, value_parser};
use std::env;
use std::path::PathBuf;
use app::{AppCommands, CRATE_REPOSITORY};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        app::show_app_desc();
        std::process::exit(0);
    }
    let arg_matches: ArgMatches = parse_args();
    let subcommand_name = arg_matches.subcommand_name().unwrap_or("");
    let app_command = AppCommands::from_str(subcommand_name);
    app::show_banner_with_starttime();
    check_deps();
    match app_command {
        Some(AppCommands::PortScan) => {
            handler::port::handle_portscan(&arg_matches);
        }
        Some(AppCommands::HostScan) => {
            handler::host::handle_hostscan(&arg_matches);
        }
        Some(AppCommands::Ping) => {
            handler::ping::handle_ping(&arg_matches);
        }
        Some(AppCommands::Trace) => {
            handler::trace::handle_traceroute(&arg_matches);
        }
        Some(AppCommands::Subdomain) => {
            handler::dns::handle_subdomain_scan(&arg_matches);
        }
        Some(AppCommands::Neighbor) => {
            handler::neighbor::handle_neighbor_discovery(&arg_matches);
        }
        Some(AppCommands::Interfaces) => {
            handler::interface::show_interfaces(&arg_matches);
        }
        Some(AppCommands::Interface) => {
            handler::interface::show_default_interface(&arg_matches);
        }
        Some(AppCommands::CheckDependencies) => {
            handler::update::check_dependencies(&arg_matches);
        }
        None => {
            match arg_matches.get_one::<String>("target") {
                Some(target_host) => {
                    if nerum_core::host::is_valid_target(target_host) {
                        handler::default_probe(target_host, &arg_matches);
                    } else {
                        app::show_app_desc();
                    }
                },
                None => {
                    app::show_app_desc();
                },
            }
        }
    }
}

fn parse_args() -> ArgMatches {
    let app_description: &str = crate_description!();
    let app: Command = Command::new(crate_name!())
        .version(crate_version!())
        .about(format!("{} \n{}", app_description, CRATE_REPOSITORY))
        .allow_external_subcommands(true)
        .arg(Arg::new("target")
            .help("Specify the target host. IP address or Hostname")
            .short('t')
            .long("target")
            .value_name("target")
            .display_order(1)
            .value_parser(value_parser!(String))
        )
        .arg(Arg::new("interface")
            .help("Specify the network interface")
            .short('i')
            .long("interface")
            .value_name("interface_name")
            .display_order(2)
            .value_parser(value_parser!(String))
        )
        .arg(Arg::new("noping")
            .help("Disable initial ping")
            .long("noping")
            .num_args(0)
        )
        .arg(Arg::new("full")
            .help("Scan all ports (1-65535)")
            .short('F')
            .long("full")
            .num_args(0)
        )
        .arg(Arg::new("json")
            .help("Displays results in JSON format.")
            .short('j')
            .long("json")
            .num_args(0)
        )
        .arg(Arg::new("save")
            .help("Save scan result in JSON format - Example: -o result.json")
            .short('o')
            .long("save")
            .value_name("file_path")
            .value_parser(value_parser!(PathBuf))
        )
        .subcommand(Command::new("port")
            .about("Scan port. nerum port --help for more information")
            .arg(Arg::new("target")
                .help("Specify the target. IP address or Hostname")
                .value_name("target")
                .value_parser(value_parser!(String))
                .required(true)
            )
            .arg(Arg::new("ports")
                .help("Specify the ports. Example: 80,443,8080")
                .short('p')
                .long("ports")
                .value_name("ports")
                .value_delimiter(',')
                .value_parser(value_parser!(u16))
            )
            .arg(Arg::new("range")
                .help("Specify the port range. Example: 1-100")
                .short('r')
                .long("range")
                .value_name("range")
                .value_delimiter('-')
                .value_parser(value_parser!(u16))
            )
            .arg(Arg::new("scantype")
                .help("Specify the scan-type")
                .short('T')
                .long("scantype")
                .value_name("scantype")
                .value_parser(value_parser!(String))
            )
            .arg(Arg::new("service")
                .help("Enable service detection")
                .short('S')
                .long("service")
                .num_args(0)
            )
            .arg(Arg::new("random")
                .help("Don't randomize targets. By default, nerum randomizes the order of targets.")
                .short('R')
                .long("random")
                .num_args(0)
            )
            .arg(Arg::new("wellknown")
                .help("Use well-known ports")
                .short('W')
                .long("wellknown")
                .num_args(0)
            )
            .arg(Arg::new("full")
                .help("Scan all ports (1-65535)")
                .short('F')
                .long("full")
                .num_args(0)
            )
            .arg(Arg::new("noping")
                .help("Disable initial ping")
                .long("noping")
                .num_args(0)
            )
            .arg(Arg::new("timeout")
                .help("Set timeout in ms - Example: --timeout 10000")
                .long("timeout")
                .value_name("timeout")
                .value_parser(value_parser!(u64))
            )
            .arg(Arg::new("waittime")
                .help("Set wait-time in ms (default:100ms) - Example: -w 200")
                .short('w')
                .long("waittime")
                .value_name("waittime")
                .value_parser(value_parser!(u64))
            )
            .arg(Arg::new("rate")
                .help("Set send-rate in ms - Example: --rate 1")
                .long("rate")
                .value_name("duration")
                .value_parser(value_parser!(u64))
            )
        )
        .subcommand(Command::new("host")
            .about("Scan host in specified network or host-list. nerum host --help for more information")
            .arg(Arg::new("target")
                .help("Specify the target network")
                .value_name("target")
                .required(true)
            )
            .arg(Arg::new("protocol")
                .help("Specify the protocol")
                .short('P')
                .long("protocol")
                .value_name("protocol_name")
                .value_parser(value_parser!(String))
            )
            .arg(Arg::new("port")
                .help("Specify the port. Example: --port 80")
                .short('p')
                .long("port")
                .value_name("port")
                .value_parser(value_parser!(u16))
            )
            .arg(Arg::new("random")
                .help("Don't randomize targets. By default, nerum randomizes the order of targets.")
                .short('R')
                .long("random")
                .num_args(0)
            )
            .arg(Arg::new("timeout")
                .help("Set timeout in ms - Example: --timeout 10000")
                .long("timeout")
                .value_name("timeout")
                .value_parser(value_parser!(u64))
            )
            .arg(Arg::new("waittime")
                .help("Set wait-time in ms (default:100ms) - Example: -w 200")
                .short('w')
                .long("waittime")
                .value_name("waittime")
                .value_parser(value_parser!(u64))
            )
            .arg(Arg::new("rate")
                .help("Set send-rate in ms - Example: --rate 1")
                .long("rate")
                .value_name("duration")
                .value_parser(value_parser!(u64))
            )
        )
        .subcommand(Command::new("ping")
            .about("Ping to specified host. nerum ping --help for more information")
            .arg(Arg::new("target")
                .help("Specify the target. IP address or Hostname")
                .value_name("target")
                .required(true)
            )
            .arg(Arg::new("count")
                .help("Set number of requests or pings to be sent")
                .short('c')
                .long("count")
                .value_name("count")
                .value_parser(value_parser!(u32))
            )
            .arg(Arg::new("port")
                .help("Specify the port. Example: 80")
                .short('p')
                .long("port")
                .value_name("port")
                .value_parser(value_parser!(u16))
                .required(false)
            )
            .arg(Arg::new("maxhop")
                .help("Set max hop(TTL) for ping or traceroute")
                .long("maxhop")
                .value_name("maxhop")
                .value_parser(value_parser!(u8))
            )
            .arg(Arg::new("protocol")
                .help("Specify the protocol")
                .short('P')
                .long("protocol")
                .value_name("protocol_name")
                .value_parser(value_parser!(String))
            )
            .arg(Arg::new("timeout")
                .help("Set timeout in ms - Example: --timeout 10000")
                .long("timeout")
                .value_name("timeout")
                .value_parser(value_parser!(u64))
            )
            .arg(Arg::new("waittime")
                .help("Set wait-time in ms (default:100ms) - Example: -w 200")
                .short('w')
                .long("waittime")
                .value_name("waittime")
                .value_parser(value_parser!(u64))
            )
            .arg(Arg::new("rate")
                .help("Set send-rate in ms - Example: --rate 100")
                .long("rate")
                .value_name("duration")
                .value_parser(value_parser!(u64))
            )
        )
        .subcommand(Command::new("trace")
            .about("Traceroute to specified host. nerum trace --help for more information")
            .arg(Arg::new("target")
                .help("Specify the target. IP address or Hostname")
                .value_name("target")
                .required(true)
            )
            .arg(Arg::new("port")
                .help("Specify the port. Example: 33435")
                .short('p')
                .long("port")
                .value_name("port")
                .value_parser(value_parser!(u16))
                .required(false)
            )
            .arg(Arg::new("maxhop")
                .help("Set max hop(TTL) for ping or traceroute")
                .long("maxhop")
                .value_name("maxhop")
                .value_parser(value_parser!(u8))
            )
            .arg(Arg::new("timeout")
                .help("Set timeout in ms - Example: --timeout 10000")
                .long("timeout")
                .value_name("timeout")
                .value_parser(value_parser!(u64))
            )
            .arg(Arg::new("waittime")
                .help("Set wait-time in ms (default:100ms) - Example: -w 200")
                .short('w')
                .long("waittime")
                .value_name("waittime")
                .value_parser(value_parser!(u64))
            )
            .arg(Arg::new("rate")
                .help("Set send-rate in ms - Example: --rate 100")
                .long("rate")
                .value_name("duration")
                .value_parser(value_parser!(u64))
            )
        )
        .subcommand(Command::new("subdomain")
            .about("Find subdomains. nerum subdomain --help for more information")
            .arg(Arg::new("target")
                .help("Specify the target apex-domain")
                .value_name("target")
                .required(true)
            )
            .arg(Arg::new("wordlist")
                .help("Specify the wordlist file path")
                .short('w')
                .long("wordlist")
                .value_name("file_path")
                .value_parser(value_parser!(PathBuf))
            )
            .arg(Arg::new("timeout")
                .help("Set timeout in ms - Example: --timeout 10000")
                .long("timeout")
                .value_name("timeout")
                .value_parser(value_parser!(u64))
            )
        )
        .subcommand(Command::new("nei")
            .about("Resolve IP address to MAC address")
            .arg(Arg::new("target")
                .help("Specify the target IP address")
                .value_name("target")
                .required(true)
            )
            .arg(Arg::new("count")
                .help("Set number of requests or pings to be sent")
                .short('c')
                .long("count")
                .value_name("count")
                .value_parser(value_parser!(u32))
            )
            .arg(Arg::new("timeout")
                .help("Set timeout in ms - Example: --timeout 10000")
                .long("timeout")
                .value_name("timeout")
                .value_parser(value_parser!(u64))
            )
            .arg(Arg::new("waittime")
                .help("Set wait-time in ms (default:100ms) - Example: -w 200")
                .short('w')
                .long("waittime")
                .value_name("waittime")
                .value_parser(value_parser!(u64))
            )
            .arg(Arg::new("rate")
                .help("Set send-rate in ms - Example: --rate 1")
                .long("rate")
                .value_name("duration")
                .value_parser(value_parser!(u64))
            )
        )
        .subcommand(Command::new("interfaces")
            .about("Show network interfaces")
        )
        .subcommand(Command::new("interface")
            .about("Show default network interface")
        )
        .subcommand(Command::new("check")
            .about("Check dependencies (Windows only)")
        )
        ;
    app.get_matches()
}

fn check_deps() {
    if cfg!(target_os = "windows"){
        if !nerum_core::sys::dep::check_dependencies() {
            println!("Npcap is not installed.");
            println!("On Windows, Npcap is required for some features.");
            println!("You can check installation by 'nerum check' command. Or Please install Npcap from https://npcap.com/#download");
            println!("Exiting...");
            std::process::exit(1);
        }
    }
}
