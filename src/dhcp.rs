#[path = "./dhcp_packet.rs"]
mod dhcp_packet;
use dhcp_packet::{Op, HType, Options, MutableDhcpPacket};

#[path = "./broadcast.rs"]
mod broadcast;
use broadcast::send_broadcast;

use pnet::datalink;
use pnet::datalink::NetworkInterface;

use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub fn run_client(interface_name: &str) {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == *interface_name)
        .expect("Failed to get interface");

    let payload: Vec<u8> = vec![0x32u8; 16];

    send_discover(&interface);
}

pub fn assemble_discover(interface: &NetworkInterface) -> MutableDhcpPacket {
    let mut discover_buffer: Vec<u8> = vec![0; MutableDhcpPacket::non_option_packet_size()];
    let mut discover_packet = MutableDhcpPacket::new(&mut discover_buffer).expect("");

    discover_packet.set_op(Op::BOOTREQUEST);
    discover_packet.set_htype(HType::Ethernet);
    discover_packet.set_hlen(6);

    let mut csprng = ChaCha20Rng::from_entropy();
    let mut xid_array = [0u8; 4];
    csprng.fill_bytes(&mut xid_array);
    discover_packet.set_xid(0x0100000 * (xid_array[0] as u32) + 0x010000 * (xid_array[1] as u32) + 0x01000 * (xid_array[2] as u32) + xid_array[3] as u32);

    let mut chaddr = [0u8; 16];
    let mac = interface.mac.unwrap();
    chaddr[0] = mac.0;
    chaddr[1] = mac.1;
    chaddr[2] = mac.2;
    chaddr[3] = mac.3;
    chaddr[4] = mac.4;
    chaddr[5] = mac.5;
    discover_packet.set_chaddr(chaddr);

    discover_packet.add_options(Options::MAGICCOOKIE.to_vec());
    discover_packet.add_options(Options::DHCPDISCOVER.to_vec());
    discover_packet.add_options(Options::PARAM.to_vec());
    discover_packet.add_options(Options::END.to_vec());
    discover_packet.add_options(Options::build_padding(&discover_packet.packet()));

    discover_packet
}

pub fn send_discover(interface: &NetworkInterface) {
    let discover_packet = assemble_discover(interface);

    println!("{:?}", discover_packet);

    send_broadcast(68, 67, interface, &discover_packet.packet());
}
