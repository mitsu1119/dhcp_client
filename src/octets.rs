type Octet = Octets<1>;

#[derive(Debug)]
pub struct Octets<const N: usize> {
    data: [u8; N]
}

impl<const N: usize> Octets<N> {
    pub fn new() -> Self {
        Self { data: [0u8; N] }
    }

    pub fn set(&mut self, data: [u8; N]) {
        self.data = data;
    }
}
