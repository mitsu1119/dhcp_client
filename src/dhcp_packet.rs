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
    xid: [u8; XID_LEN],
    ciaddr: [u8; CIADDR_LEN],
    yiaddr: [u8; YIADDR_LEN],
    chaddr: [u8; CHADDR_LEN]
}

impl DhcpPacket {
    pub fn new() -> Self {
        Self {
            op: Op::BOOTREQUEST,
            htype: Htype::Ethernet,
            hlen: 0,
            xid: [0; XID_LEN],
            ciaddr: [0; CIADDR_LEN],
            yiaddr: [0; YIADDR_LEN],
            chaddr: [0; CHADDR_LEN]
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
