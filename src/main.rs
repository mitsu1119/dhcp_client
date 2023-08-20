use std::env;

use log::*;
use std::error::Error;
use anyhow::{self};

use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

use std::net::UdpSocket;

mod dhcp_packet;
use dhcp_packet::*;

use crate::octets::Octets;

// DHCP DISCOVER のパケットを構築
fn build_discover() {
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

    println!("{:?}", dhcp_packet);
}

// 利用可能な DHCP サーバを探す
fn dhcp_discover() -> anyhow::Result<()> {
    // 67 番ポートにブロードキャスト
    const address: &str = "0.0.0.0:67";

    let socket = match UdpSocket::bind("127.0.0.1:68") {
        Ok(v) => v,
        Err(e) => {
            error!("connect: {}", e);
            std::process::exit(1);
        }
    };

    // DHCP DISCOVER を構築
    build_discover();

    // DHCP DISCOVER を送信

    // サーバからの返答を受信

    Ok(())
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    dhcp_discover();
}
