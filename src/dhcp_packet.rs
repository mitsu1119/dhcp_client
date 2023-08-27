// TODO: options は一旦マジッククッキーだけ
#[derive(Debug)]
pub struct MutableDhcpPacket<'a> {
    buffer: &'a mut Vec<u8>,
    op:     Op,
    htype:  HType,
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

impl MutableDhcpPacket<'_> {
    fn new(buffer: &mut Vec<u8>) {
    }

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
