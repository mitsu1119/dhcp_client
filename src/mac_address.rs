use std::fs::File;
use std::path::Path;
use std::io::Read;

// MAC アドレスの取得
pub fn get_mac(interface_name: &str) -> anyhow::Result<[u8; 6]> {

    let interface_addr_file = format!("/sys/class/net/{}/address", interface_name);
    let path = Path::new(&interface_addr_file);

    let mut f = File::open(path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let bytes: Vec<u8> = contents.trim().split(':').map(|x| u8::from_str_radix(x, 16).unwrap()).collect();
    let res = bytes[..6].try_into()?;

    Ok(res)
}
