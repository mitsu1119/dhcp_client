use pnet::datalink;
use pnet::datalink::Channel;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink::NetworkInterface;
use pnet::packet::ethernet::{MutableEthernetPacket, EtherTypes, EthernetPacket};

use pnet::packet::Packet;
use pnet::util::MacAddr;

use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4;
use pnet::packet::ipv4::MutableIpv4Packet;
use std::net::Ipv4Addr;

use pnet::packet::udp;
use pnet::packet::udp::MutableUdpPacket;

// ブロードキャスト用のソケット
pub struct BroadcastSocket {
    interface: NetworkInterface,
    channel: Channel
}

impl BroadcastSocket {
    pub fn new(interface: &NetworkInterface) -> Self {
        let (tx, rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unsupported channel type"),
            Err(e) => panic!("Failed to create datalink channel {}", e)
        };

        BroadcastSocket { interface: interface.clone(), channel: Ethernet(tx, rx) }
    }

    /* UDP で payload の内容の ipv4 ブロードキャストを送る */
    pub fn send(&mut self, src_port: u16, dest_port: u16, interface: &NetworkInterface, payload: &Vec<u8>) {
        let mut udp_buffer: Vec<u8> = vec![0; MutableUdpPacket::minimum_packet_size() + payload.len()];
        let mut udp_packet = MutableUdpPacket::new(&mut udp_buffer).unwrap();

        udp_packet.set_source(src_port);
        udp_packet.set_destination(dest_port);
        udp_packet.set_length((MutableUdpPacket::minimum_packet_size() + payload.len()).try_into().unwrap());
        udp_packet.set_payload(payload);
        udp_packet.set_checksum(0);
        udp_packet.set_checksum(udp::ipv4_checksum(&udp_packet.to_immutable(), &Ipv4Addr::new(0, 0, 0, 0), &Ipv4Addr::new(255, 255, 255, 255)));

        let payload = build_ipv4_header(interface, &udp_packet.packet().to_vec());

        let Ethernet(ref mut tx, _) = self.channel else { panic!(""); };
        tx.send_to(&payload, None).expect("Failed to send");
    }

    /* 自分の MAC アドレス宛のフレームを一つ受信して func を呼び出し */
    pub fn recv_l2<T>(&mut self, func: impl Fn(EthernetPacket) -> T) -> T {
        let Ethernet(_, ref mut rx) = self.channel else { panic!(""); };
        loop {
            match rx.next() {
                Ok(frame) => {
                    let frame = EthernetPacket::new(frame).unwrap();
                    if frame.get_destination() == self.interface.mac.unwrap() {
                        return func(frame);
                    }
                },
                Err(e) => {
                    println!("Failed to read: {}", e);
                }
            }
        }
    }
}

/* レイヤ 3 で payload の内容のブロードキャストを送る */
/* IPv4 のみ対応 */
pub fn build_ipv4_header(interface: &NetworkInterface, payload: &Vec<u8>) -> Vec<u8> {
    let mut ipv4_buffer: Vec<u8> = vec![0; MutableIpv4Packet::minimum_packet_size() + payload.len()];
    let mut ipv4_packet = MutableIpv4Packet::new(&mut ipv4_buffer).unwrap();

    ipv4_packet.set_version(4);
    ipv4_packet.set_header_length((MutableIpv4Packet::minimum_packet_size() / 4).try_into().unwrap());
    ipv4_packet.set_ecn(0);
    ipv4_packet.set_dscp(4);
    ipv4_packet.set_total_length(ipv4_packet.packet().len() as u16);
    ipv4_packet.set_identification(0);
    ipv4_packet.set_flags(0);
    ipv4_packet.set_fragment_offset(0);
    ipv4_packet.set_ttl(128);
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Udp);
    ipv4_packet.set_source(Ipv4Addr::new(0, 0, 0, 0));
    ipv4_packet.set_destination(Ipv4Addr::new(255, 255, 255, 255));
    ipv4_packet.set_checksum(ipv4::checksum(&ipv4_packet.to_immutable()));
    ipv4_packet.set_payload(payload);

    build_ethernet_header(interface, &ipv4_packet.packet().to_vec())
}

/* レイヤ 2 で payload の内容のブロードキャストを送る */
pub fn build_ethernet_header(interface: &NetworkInterface, payload: &Vec<u8>) -> Vec<u8> {
    let mut ethernet_buffer: Vec<u8> = vec![0; MutableEthernetPacket::minimum_packet_size() + payload.len()];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    ethernet_packet.set_destination(MacAddr::broadcast());
    ethernet_packet.set_source(interface.mac.unwrap());
    ethernet_packet.set_ethertype(EtherTypes::Ipv4);
    ethernet_packet.set_payload(payload);

    ethernet_packet.packet().to_vec()
}
