// TODO: options は一旦マジッククッキーだけ
#[derive(Debug)]
pub struct MutableDhcpPacket {
    op:     Op,
    htype:  Htype,
    hlen:   u8,
    hops:   u8,
    xid:    u32,
    secs:   u16,
    flags:  u16,
    ciaddr: u32,
    yiaddr: u32,
    siaddr: u32,
    giaddr: u32,
    chaddr: u128,
    sname:  [u8; 64],
    file:   [u8; 128],
    options:u8
}

impl MutableDhcpPacket {
    fn minimum_packet_size() -> usize {
        237
    }
}

#[repr(u8)]
#[derive(Debug)]
pub enum Op {
    BOOTREQUEST = 1,
    BOOTREPLY   = 2
}

#[repr(u8)]
#[derive(Debug)]
pub enum HType {
    Ethernet = 1
}
