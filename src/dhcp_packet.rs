use std::collections::VecDeque;

#[derive(Debug)]
pub struct MutableDhcpPacket {
    op:     u8,
    htype:  u8,
    hlen:   u8,
    hops:   u8,
    xid:    u32,
    secs:   u16,
    flags:  u16,
    ciaddr: u32,
    yiaddr: u32,
    siaddr: u32,
    giaddr: u32,
    chaddr: Vec<u8>,
    sname:  Vec<u8>,
    file:   Vec<u8>,
    options:Vec<Vec<u8>>
}

impl MutableDhcpPacket {
    pub fn new(buffer: &mut Vec<u8>) -> Result<Self, &str> {
        if buffer.len() < Self::non_option_packet_size() {
            return Err("Buffer for dhcp packet is too short.")
        }

        let op = buffer[0];
        let htype = buffer[1];
        let hlen = buffer[2];
        let hops = buffer[3];
        let xid = u32::from_be_bytes(buffer[4..8].try_into().unwrap());
        let secs = u16::from_be_bytes(buffer[8..10].try_into().unwrap());
        let flags = u16::from_be_bytes(buffer[10..12].try_into().unwrap());
        let ciaddr = u32::from_be_bytes(buffer[12..16].try_into().unwrap());
        let yiaddr = u32::from_be_bytes(buffer[16..20].try_into().unwrap());
        let siaddr = u32::from_be_bytes(buffer[20..24].try_into().unwrap());
        let giaddr = u32::from_be_bytes(buffer[24..28].try_into().unwrap());
        let chaddr = buffer[28..44].to_vec();
        let sname = buffer[44..108].to_vec();
        let file = buffer[108..236].to_vec();

        let mut options: Vec<Vec<u8>> = vec![];
        let mut ops: VecDeque<u8> = buffer[Self::non_option_packet_size()..].to_vec().into();

        let mut padding: Vec<u8> = vec![];
        while ops.len() != 0 {
            if ops[0] != 0x00 && padding.len() != 0 {
                options.push(padding);
                padding = vec![];
            }
            match ops[0] {
                // magic cookie
                0x63 => options.push(ops.drain(..4).collect::<Vec<u8>>()),
                // pad
                0x00 => { padding.push(0u8); ops.pop_front(); },
                // end
                0xff => options.push(ops.drain(..1).collect::<Vec<u8>>()),
                other => options.push(ops.drain(..(ops[1]+2) as usize).collect::<Vec<u8>>()),
            };
            println!("{:?}", options);
        }
        if padding.len() != 0 { options.push(padding); }

        let res = MutableDhcpPacket {
            op, htype, hlen, hops, xid, secs, flags, ciaddr, yiaddr, siaddr, giaddr, chaddr, sname, file, options
        };

        Ok(res)
    }

    pub fn packet(&self) -> Vec<u8> {
        let mut res = vec![];
        res.push(self.op);
        res.push(self.htype);
        res.push(self.hlen);
        res.push(self.hops);
        res = [res, self.xid.to_be_bytes().to_vec(), self.secs.to_be_bytes().to_vec(), self.flags.to_be_bytes().to_vec(), self.ciaddr.to_be_bytes().to_vec(), self.yiaddr.to_be_bytes().to_vec(), self.siaddr.to_be_bytes().to_vec(), self.giaddr.to_be_bytes().to_vec(), self.chaddr.clone(), self.sname.clone(), self.file.clone(), self.options.clone().into_iter().flatten().collect::<Vec<_>>()].concat();

        res
    }

    pub fn non_option_packet_size() -> usize {
        236
    }

    pub fn set_op(&mut self, op: u8) { self.op = op; }
    pub fn set_htype(&mut self, htype: u8) { self.htype = htype; }
    pub fn set_hlen(&mut self, hlen: u8) { self.hlen = hlen; }
    pub fn set_hops(&mut self, hops: u8) { self.hops = hops; }
    pub fn set_xid(&mut self, xid: u32) { self.xid = xid; }
    pub fn set_secs(&mut self, secs: u16) { self.secs = secs; }
    pub fn set_flags(&mut self, flags: u16) { self.flags = flags; }
    pub fn set_ciaddr(&mut self, ciaddr: u32) { self.ciaddr = ciaddr; }
    pub fn set_yiaddr(&mut self, yiaddr: u32) { self.yiaddr = yiaddr; }
    pub fn set_siaddr(&mut self, siaddr: u32) { self.siaddr = siaddr; }
    pub fn set_giaddr(&mut self, giaddr: u32) { self.giaddr = giaddr; }
    pub fn set_chaddr(&mut self, chaddr: [u8; 16]) { self.chaddr = chaddr.to_vec(); }
    pub fn set_sname(&mut self, sname: [u8; 64]) { self.sname = sname.to_vec(); }
    pub fn set_file(&mut self, file: [u8; 128]) { self.file = file.to_vec(); }
    pub fn add_options(&mut self, option: Vec<u8>) { self.options.push(option); }
}

pub struct Op {}
impl Op {
    pub const BOOTREQUEST: u8 = 1;
    pub const BOOTREPLY: u8 = 2;
}

pub struct HType {}
impl HType {
    pub const Ethernet: u8 = 1;
}

pub struct Options {}
impl Options {
    // magic cookie
    pub const MAGICCOOKIE: [u8; 4] = [0x63, 0x82, 0x53, 0x63];

    // dhcp message type
    pub const DHCPDISCOVER: [u8; 3] = [0x35, 0x01, 0x01];
    pub const DHCPOFFER:    [u8; 3] = [0x35, 0x01, 0x02];

    // end
    pub const END: [u8; 1] = [0xff];
}
