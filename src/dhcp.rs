#[path = "./dhcp_packet.rs"]
mod dhcp_packet;
use dhcp_packet::{Op, HType, Options, MutableDhcpPacket};

#[path = "./broadcast.rs"]
mod broadcast;
use broadcast::BroadcastSocket;

use pnet::datalink;
use pnet::datalink::NetworkInterface;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ethernet::EtherTypes;

use pnet::packet::Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use std::net::Ipv4Addr;

use pnet::packet::udp::UdpPacket;

use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

pub fn run_client(interface_name: &str) {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == *interface_name)
        .expect("Failed to get interface");

    let mut sock = BroadcastSocket::new(&interface);

    let xid = send_discover(&mut sock, &interface);
    let ip = sock.recv_l2(dhcpoffer_handler);

    send_request(&mut sock, xid, ip);
}

fn build_request(interface: &NetworkInterface, xid: u32, ip: Ipv4Addr) -> MutableDhcpPacket {
    let mut request_buffer: Vec<u8> = vec![0; MutableDhcpPacket::non_option_packet_size()];
    let mut request_packet = MutableDhcpPacket::new(&mut request_buffer).expect("");

    request_packet.set_op(Op::BOOTREQUEST);
    request_packet.set_htype(HType::ETHERNET);
    request_packet.set_hlen(6);
    request_packet.set_xid(xid);

    let mut chaddr = [0u8; 16];
    let mac = interface.mac.unwrap();
    chaddr[0] = mac.0;
    chaddr[1] = mac.1;
    chaddr[2] = mac.2;
    chaddr[3] = mac.3;
    chaddr[4] = mac.4;
    chaddr[5] = mac.5;
    request_packet.set_chaddr(chaddr);

    // TODO: requested ip address, server identifier
    request_packet.add_options(Options::MAGICCOOKIE.to_vec());
    request_packet.add_options(Options::build_message_type(Options::DHCPREQUEST).to_vec());
    request_packet.add_options(Options::END.to_vec());

    request_packet
}

/* DHCP INFORM の送信 */
fn send_request(sock: &mut BroadcastSocket, xid: u32, ip: Ipv4Addr) {
    let request_packet = build_request(sock.get_interface(), xid, ip);
    println!("{:?}", request_packet);
    // sock.send(68, 67, interface, &inform_packet.packet());
}

/* DHCPOFFER の受信 */
fn dhcpoffer_handler(frame: EthernetPacket) -> Ipv4Addr {
    if frame.get_ethertype() == EtherTypes::Ipv4 {
        // フレームを ipv4 パケットに変換
        let ipv4_packet = Ipv4Packet::new(frame.payload()).unwrap();
        if ipv4_packet.get_next_level_protocol() == IpNextHeaderProtocols::Udp {
            let packet = UdpPacket::new(ipv4_packet.payload()).unwrap();
            let mut buffer: Vec<u8> = packet.payload().to_vec();
            let dhcp_packet = MutableDhcpPacket::new(&mut buffer).unwrap();
            let options = dhcp_packet.get_options();
            for option in options.iter() {
                if option[0] == Options::MESSAGE_TYPE && option[2] == Options::DHCPOFFER {
                    let ip = dhcp_packet.get_yiaddr().to_be_bytes();
                    return Ipv4Addr::new(ip[0], ip[1], ip[2], ip[3]);
                }
            }
        }
    }
    panic!("Unsupported packet");
}

fn build_discover(interface: &NetworkInterface) -> MutableDhcpPacket {
    let mut discover_buffer: Vec<u8> = vec![0; MutableDhcpPacket::non_option_packet_size()];
    let mut discover_packet = MutableDhcpPacket::new(&mut discover_buffer).expect("");

    discover_packet.set_op(Op::BOOTREQUEST);
    discover_packet.set_htype(HType::ETHERNET);
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
    discover_packet.add_options(Options::build_message_type(Options::DHCPDISCOVER).to_vec());
    discover_packet.add_options(Options::END.to_vec());

    discover_packet
}

/* DHCP DISCOVER の送信 */
/* xid を返却 */
fn send_discover(sock: &mut BroadcastSocket, interface: &NetworkInterface) -> u32 {
    let discover_packet = build_discover(interface);
    sock.send(68, 67, interface, &discover_packet.packet());
    discover_packet.get_xid()
}
