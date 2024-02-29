use ethers::types::Address;
use std::str::FromStr;

trait EtheriumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}

impl EtheriumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Etherium"),
        }
    }
}

impl EtheriumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}

fn get_etherium_data<T: EtheriumAddress>(address: T) -> Address {
    let converted_address = address.convert_address().unwrap();
    converted_address
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_polymorphism() {
        let addr: Address =
            Address::from_str("0x48319f97E5Da1233c21c48b80097c0FB7a20Ff86").unwrap();
        assert_eq!(
            addr,
            Address::from_str("0x48319f97E5Da1233c21c48b80097c0FB7a20Ff86").unwrap()
        );
    }
}
