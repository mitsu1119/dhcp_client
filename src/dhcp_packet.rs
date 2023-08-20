#[path = "./octets.rs"]
mod octets;
use octets::Octets;

const XID_LEN: usize = 4;
const CIADDR_LEN: usize = 4;
const YIADDR_LEN: usize = 4;
const CHADDR_LEN: usize = 16;

// TODO: options は一旦無し
#[derive(Debug)]
pub struct DhcpPacket {
    op: Op,
    htype: Htype,
    hlen: u8,
    xid: Octets<XID_LEN>,
    ciaddr: Octets<CIADDR_LEN>,
    yiaddr: Octets<YIADDR_LEN>,
    chaddr: Octets<CHADDR_LEN>
}

impl DhcpPacket {
    pub fn new() -> Self {
        let mut xid = Octets::new();
        Self {
            op: Op::BOOTREQUEST,
            htype: Htype::Ethernet,
            hlen: 0,
            xid: xid,
            ciaddr: Octets::new(),
            yiaddr: Octets::new(),
            chaddr: Octets::new()
        }
    }

    pub fn set_op(&mut self, op: Op) { self.op = op; }
    pub fn set_htype(&mut self, htype: Htype) { self.htype = htype; }
    pub fn calc_and_set_hlen(&mut self) { 
        self.hlen = match self.htype {
            Htype::Ethernet => 6,
        }
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
pub enum Htype {
    Ethernet
}
