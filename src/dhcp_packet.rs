// TODO: options は一旦マジッククッキーだけ
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
    options:u8
}

impl MutableDhcpPacket {
    pub fn new(buffer: &mut Vec<u8>) -> Result<Self, &str> {
        if buffer.len() < Self::minimum_packet_size() {
            return Err("Buffer for dhcp packet is too short.")
        }

        let options = buffer.pop().unwrap();
        let file = buffer.drain(108..).collect::<Vec<u8>>();
        let sname = buffer.drain(44..).collect::<Vec<u8>>();
        let chaddr = buffer.drain(28..).collect::<Vec<u8>>();
        let giaddr = buffer.pop().unwrap() as u32 + 0x10 * (buffer.pop().unwrap() as u32) + 0x100 * (buffer.pop().unwrap() as u32) + 0x1000 * (buffer.pop().unwrap() as u32);
        let siaddr = buffer.pop().unwrap() as u32 + 0x10 * (buffer.pop().unwrap() as u32) + 0x100 * (buffer.pop().unwrap() as u32) + 0x1000 * (buffer.pop().unwrap() as u32);
        let yiaddr = buffer.pop().unwrap() as u32 + 0x10 * (buffer.pop().unwrap() as u32) + 0x100 * (buffer.pop().unwrap() as u32) + 0x1000 * (buffer.pop().unwrap() as u32);
        let ciaddr = buffer.pop().unwrap() as u32 + 0x10 * (buffer.pop().unwrap() as u32) + 0x100 * (buffer.pop().unwrap() as u32) + 0x1000 * (buffer.pop().unwrap() as u32);
        let flags = 0x10 * (buffer.pop().unwrap() as u16) + buffer.pop().unwrap() as u16;
        let secs = 0x10 * (buffer.pop().unwrap() as u16) + buffer.pop().unwrap() as u16;
        let xid = buffer.pop().unwrap() as u32 + 0x10 * (buffer.pop().unwrap() as u32) + 0x100 * (buffer.pop().unwrap() as u32) + 0x1000 * (buffer.pop().unwrap() as u32);
        let hops = buffer.pop().unwrap();
        let hlen = buffer.pop().unwrap();
        let htype = buffer.pop().unwrap();
        let op = buffer.pop().unwrap();

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
        res = [res, self.xid.to_be_bytes().to_vec(), self.secs.to_be_bytes().to_vec(), self.flags.to_be_bytes().to_vec(), self.ciaddr.to_be_bytes().to_vec(), self.yiaddr.to_be_bytes().to_vec(), self.siaddr.to_be_bytes().to_vec(), self.giaddr.to_be_bytes().to_vec(), self.chaddr.clone(), self.sname.clone(), self.file.clone()].concat();
        res.push(self.options);

        res
    }

    pub fn minimum_packet_size() -> usize {
        237
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
    pub fn set_options(&mut self, options: u8) { self.options = options; }
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
