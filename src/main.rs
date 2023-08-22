use std::{env, str};

use log::*;
use std::error::Error;
use anyhow::{self};

use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod dhcp_packet;
use dhcp_packet::*;

mod mac_address;
use mac_address::*;

use crate::octets::Octets;

// DHCP DISCOVER のパケットを構築
fn build_discover(interface_name: &str) -> Vec<u8>{
    let mut dhcp_packet = DhcpPacket::new();

    dhcp_packet.set_op(Op::BOOTREQUEST);
    dhcp_packet.set_htype(Htype::Ethernet);
    dhcp_packet.calc_and_set_hlen();
    dhcp_packet.set_hops(0);

    let mut csprng = ChaCha20Rng::from_entropy();
    let mut xid_array = [0u8; XID_LEN];
    csprng.fill_bytes(&mut xid_array);
    let mut xid = Octets::<XID_LEN>::new();
    xid.set(xid_array);
    dhcp_packet.set_xid(xid);

    let mut secs = Octets::<SECS_LEN>::new();
    secs.from_num(0);
    dhcp_packet.set_secs(secs);

    dhcp_packet.set_flags(Flags::Broadcast);
    dhcp_packet.set_ciaddr(Octets::<CIADDR_LEN>::new());
    dhcp_packet.set_yiaddr(Octets::<CIADDR_LEN>::new());
    dhcp_packet.set_siaddr(Octets::<CIADDR_LEN>::new());
    dhcp_packet.set_giaddr(Octets::<CIADDR_LEN>::new());

    let mac_address = get_mac(interface_name).expect("Could not get mac address");
    let chaddr_padding = [0u8; 10];
    let chaddr_vec: Vec<u8> = mac_address.iter().chain(&chaddr_padding).map(|&x| x).collect();
    let chaddr_array: [u8; CHADDR_LEN] = chaddr_vec.try_into().expect("Invalid mac address syntax");
    let mut chaddr = Octets::<CHADDR_LEN>::new();
    chaddr.set(chaddr_array);
    dhcp_packet.set_chaddr(chaddr);

    println!("{:?}", dhcp_packet);
    println!("{:?}", dhcp_packet.get_bytes());

    dhcp_packet.get_bytes().to_vec()
}

// 利用可能な DHCP サーバを探す
fn dhcp_discover(interface_name: &str) -> anyhow::Result<()> {
    // 67 番ポートにブロードキャスト
    let client_address = Ipv4Addr::new(0, 0, 0, 0);
    let server_address = Ipv4Addr::new(255, 255, 255, 255);

    // DHCP DISCOVER を構築
    let payload = build_discover(interface_name);

    // DHCP DISCOVER を送信
    println!("{:?}", payload.len());
    // socket.send_to(&payload, address);

    println!("{:?}", pay);
    socket.send_to(pay, address).expect("cannnot send");

    // サーバからの返答を受信
    const MAX_BUFFER: usize = 1024;
    let mut buffer = [0u8; MAX_BUFFER];
    socket.recv_from(&mut buffer)?;
    print!("retn {}", str::from_utf8(&mut buffer)?);

    loop {
    }

    Ok(())
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("Please specify [interface name].");
        std::process::exit(1);
    }
    let interface_name = &args[1];

    dhcp_discover(interface_name);
}
