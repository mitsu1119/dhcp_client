#[path = "./dhcp_packet.rs"]
mod dhcp_packet;
use dhcp_packet::MutableDhcpPacket;

pub fn send_discover() {
    let mut discover_buffer: Vec<u8> = vec![0; MutableDhcpPacket::minimum_packet_size()];
    let mut discover_packet = MutableDhcpPacket::new(&mut discover_buffer);

    println!("{:?}", discover_packet);
}
