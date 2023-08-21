#[path = "./octets.rs"]
pub mod octets;
use octets::Octets;

pub const XID_LEN:    usize = 4;
pub const SECS_LEN:   usize = 2;
pub const FLAGS_LEN:  usize = 2;
pub const CIADDR_LEN: usize = 4;
pub const YIADDR_LEN: usize = 4;
pub const SIADDR_LEN: usize = 4;
pub const GIADDR_LEN: usize = 4;
pub const CHADDR_LEN: usize = 16;
pub const SNAME_LEN:  usize = 64;
pub const FILE_LEN:   usize = 128;

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
            flags:  Flags::Unicast,
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
    pub fn set_hops(&mut self, hops: u8) { self.hops = hops; }
    pub fn set_xid(&mut self, xid: Octets<XID_LEN>) { self.xid = xid; }
    pub fn set_secs(&mut self, secs: Octets<SECS_LEN>) { self.secs = secs; }
    pub fn set_flags(&mut self, flags: Octets<FLAGS_LEN>) { self.flags = flags; }
    pub fn set_ciaddr(&mut self, ciaddr: Octets<CIADDR_LEN>) { self.ciaddr = ciaddr; }
    pub fn set_yiaddr(&mut self, yiaddr: Octets<CIADDR_LEN>) { self.yiaddr = yiaddr; }
    pub fn set_siaddr(&mut self, siaddr: Octets<CIADDR_LEN>) { self.siaddr = siaddr; }
    pub fn set_giaddr(&mut self, giaddr: Octets<CIADDR_LEN>) { self.giaddr = giaddr; }
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
    Ethernet = 1
}

#[derive(Debug)]
pub struct Flags {}

impl Flags {
    pub const Unicast: Octets<FLAGS_LEN> = Octets::<FLAGS_LEN> { data: [0u8; FLAGS_LEN] };
    pub const Broadcast: Octets<FLAGS_LEN> = Octets::<FLAGS_LEN> { data: [0b01000000, 0] };
}
