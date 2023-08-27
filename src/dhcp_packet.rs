// TODO: options は一旦マジッククッキーだけ
#[derive(Debug)]
pub struct MutableDhcpPacket {
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
        let htype = HType::try_from(buffer.pop().unwrap()).expect("");
        let op = Op::try_from(buffer.pop().unwrap()).expect("");

        let res = MutableDhcpPacket {
            op, htype, hlen, hops, xid, secs, flags, ciaddr, yiaddr, siaddr, giaddr, chaddr, sname, file, options
        };

        Ok(res)
    }

    pub fn packet(&self) -> Vec<u8> {
        let mut res = vec![];
        res.push(self.op.get_code());
        res.push(self.htype.get_code());
        res.push(self.hlen);
        res.push(self.hops);
        res = [res, self.xid.to_be_bytes().to_vec(), self.secs.to_be_bytes().to_vec(), self.flags.to_be_bytes().to_vec(), self.ciaddr.to_be_bytes().to_vec(), self.yiaddr.to_be_bytes().to_vec(), self.siaddr.to_be_bytes().to_vec(), self.giaddr.to_be_bytes().to_vec(), self.chaddr.clone(), self.sname.clone(), self.file.clone()].concat();
        res.push(self.options);

        res
    }

    pub fn minimum_packet_size() -> usize {
        237
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum Op {
    BOOTREQUEST,
    BOOTREPLY
}

impl Op {
    fn get_code(&self) -> u8 {
        match self {
            Self::BOOTREQUEST => 1,
            Self::BOOTREPLY => 2
        }
    }
}

impl TryFrom<u8> for Op {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::BOOTREQUEST),
            1 => Ok(Self::BOOTREPLY),
            _ => Err("Cannot convert to Op")
        }
    }
}

impl From<Op> for u8 {
    fn from(value: Op) -> u8 {
        match value {
            Op::BOOTREQUEST => 0,
            Op::BOOTREPLY => 1 
        }
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum HType {
    Ethernet = 0
}

impl HType {
    fn get_code(&self) -> u8 {
        match self {
            Self::Ethernet => 1
        }
    }
}

impl TryFrom<u8> for HType {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Ethernet),
            _ => Err("Cannot convert to HType")
        }
    }
}

impl From<HType> for u8 {
    fn from(value: HType) -> u8 {
        match value {
            HType::Ethernet => 0
        }
    }
}
