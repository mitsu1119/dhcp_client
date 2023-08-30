mod dhcp;

fn main() {
    /*
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("Please specify [interface name].");
        std::process::exit(1);
    }
    let interface_name = &args[1];
    */
    let interface_name = "enp0s31f6";

    dhcp::run_client(interface_name);
}
