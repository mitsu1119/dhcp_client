type Octet = Octets<1>;

#[derive(Debug)]
pub struct Octets<const N: usize> {
    pub data: [u8; N]
}

impl<const N: usize> Octets<N> {
    pub fn new() -> Self {
        Self { data: [0u8; N] }
    }

    pub fn set(&mut self, data: [u8; N]) {
        self.data = data;
    }

    pub fn get_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl Octets<2> {
    pub fn from_num(&mut self, val: u16) {
        let mut num = val;
        self.data[1] = (num % 0xff) as u8;
        num /= 0xff;
        self.data[0] = (num % 0xff) as u8;
    }
}

impl Octets<4> {
    pub fn from_num(&mut self, val: u32) {
        let mut num = val;
        self.data[3] = (num % 0xff) as u8;
        num /= 0xff;
        self.data[2] = (num % 0xff) as u8;
        num /= 0xff;
        self.data[1] = (num % 0xff) as u8;
        num /= 0xff;
        self.data[0] = (num % 0xff) as u8;
    }
}
