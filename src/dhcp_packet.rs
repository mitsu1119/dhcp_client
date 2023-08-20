#[path = "./octets.rs"]
mod octets;
use octets::Octets;

const XID_LEN:    usize = 4;
const SECS_LEN:   usize = 2;
const FLAGS_LEN:  usize = 2;
const CIADDR_LEN: usize = 4;
const YIADDR_LEN: usize = 4;
const SIADDR_LEN: usize = 4;
const GIADDR_LEN: usize = 4;
const CHADDR_LEN: usize = 16;
const SNAME_LEN:  usize = 64;
const FILE_LEN:   usize = 128;

// TODO: options は一旦無し
#[derive(Debug)]
pub struct DhcpPacket {
    op:     Op,
    htype:  Htype,
    hlen:   u8,
    hops:   u8,
    xid:    Octets<XID_LEN>,
    secs:   Octets<SECS_LEN>,
    flags:  Octets<FLAGS_LEN>,
    ciaddr: Octets<CIADDR_LEN>,
    yiaddr: Octets<YIADDR_LEN>,
    siaddr: Octets<SIADDR_LEN>,
    giaddr: Octets<GIADDR_LEN>,
    chaddr: Octets<CHADDR_LEN>,
    sname:  Octets<SNAME_LEN>,
    file:   Octets<FILE_LEN>
}

impl DhcpPacket {
    pub fn new() -> Self {
        Self {
            op:     Op::BOOTREQUEST,
            htype:  Htype::Ethernet,
            hlen:   0,
            hops:   0,
            xid:    Octets::new(),
            secs:   Octets::new(),
            flags:  Octets::new(),
            ciaddr: Octets::new(),
            yiaddr: Octets::new(),
            siaddr: Octets::new(),
            giaddr: Octets::new(),
            chaddr: Octets::new(),
            sname:  Octets::new(),
            file:   Octets::new()
        }
    }

    pub fn set_op(&mut self, op: Op) { self.op = op; }
    pub fn set_htype(&mut self, htype: Htype) { self.htype = htype; }
    pub fn calc_and_set_hlen(&mut self) { 
        self.hlen = match self.htype {
            Htype::Ethernet => 6,
        }
    }
    pub fn set_hops(&mut self, hop: u8) { self.hops = hops; }
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
