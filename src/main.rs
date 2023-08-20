use std::{io, env, str};

use log::*;
use std::error::Error;
use anyhow::{self};

use std::net::UdpSocket;

// 利用可能な DHCP サーバを探す
pub fn dhcp_discover() -> anyhow::Result<()> {
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

    // DHCP DISCOVER を送信

    // サーバからの返答を受信
}

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    dhcp_discover();
}
