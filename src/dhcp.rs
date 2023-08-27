#[path = "./dhcp_packet.rs"]
mod dhcp_packet;
use dhcp_packet::{Op, HType, MutableDhcpPacket};

use pnet::datalink::NetworkInterface;

use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub fn assemble_discover(interface: &NetworkInterface) -> MutableDhcpPacket {
    let mut discover_buffer: Vec<u8> = vec![0; MutableDhcpPacket::minimum_packet_size()];
    let mut discover_packet = MutableDhcpPacket::new(&mut discover_buffer).expect("");

    discover_packet.set_op(Op::BOOTREQUEST);
    discover_packet.set_htype(HType::Ethernet);
    discover_packet.set_hlen(6);

    let mut csprng = ChaCha20Rng::from_entropy();
    let mut xid_array = [0u8; 4];
    csprng.fill_bytes(&mut xid_array);
    discover_packet.set_xid(0x1000 * (xid_array[0] as u32) + 0x100 * (xid_array[1] as u32) + 0x10 * (xid_array[2] as u32) + xid_array[3] as u32);

    discover_packet.set_options(0x63825363);

    discover_packet
}

pub fn send_discover(interface: &NetworkInterface) {
    let discover_packet = assemble_discover(interface);

    println!("{:?}", discover_packet);
    println!("{:?}", discover_packet.packet());
}
