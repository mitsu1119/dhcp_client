#[path = "./dhcp_packet.rs"]
mod dhcp_packet;
use dhcp_packet::{Op, HType, MutableDhcpPacket};

pub fn assemble_discover() -> MutableDhcpPacket {
    let mut discover_buffer: Vec<u8> = vec![0; MutableDhcpPacket::minimum_packet_size()];
    let mut discover_packet = MutableDhcpPacket::new(&mut discover_buffer).expect("");

    discover_packet.set_op(Op::BOOTREQUEST);
    discover_packet.set_htype(HType::Ethernet);
    discover_packet.set_hlen(6);

    discover_packet.set_options(0x63825363);

    discover_packet
}

pub fn send_discover() {
    let discover_packet = assemble_discover();

    println!("{:?}", discover_packet);
    println!("{:?}", discover_packet.packet());
}
