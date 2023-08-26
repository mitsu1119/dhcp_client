use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::NetworkInterface;
use pnet::packet::ethernet::{MutableEthernetPacket, EtherTypes};

use pnet::packet::Packet;
use pnet::util::MacAddr;

pub fn send_broadcast_l3(src_port: u16, dest_port: u16, interface: &NetworkInterface, payload: &Vec<u8>) {
    send_broadcast_l2(interface, payload);
}

pub fn send_broadcast_l2(interface: &NetworkInterface, payload: &Vec<u8>) {
    let (mut tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("Failed to create datalink channel {}", e)
    };

    println!("{}", MutableEthernetPacket::minimum_packet_size());
    let mut ethernet_buffer: Vec<u8> = vec![0; MutableEthernetPacket::minimum_packet_size() + payload.len()];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    ethernet_packet.set_destination(MacAddr::broadcast());
    ethernet_packet.set_source(interface.mac.unwrap());
    ethernet_packet.set_ethertype(EtherTypes::Ipv4);

    tx.send_to(ethernet_packet.packet(), None).expect("wow");
}
