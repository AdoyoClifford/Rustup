use ethers::types::Address;
use std::str::FromStr;


trait EtheriumAddress {
    fn convert_address(&self) -> Result<Address, &'static str>;
}

impl EtheriumAddress for &str {
    fn convert_address(&self) -> Result<Address, &'static str> {
        match Address::from_str(self) {
            Ok(address) => Ok(address),
            Err(_) => Err("Invalid Etherium")
        }
    }
    
}

impl EtheriumAddress for Address {
    fn convert_address(&self) -> Result<Address, &'static str> {
        Ok(*self)
    }
}


fn get_etherium_data<T>(address: T) {

}