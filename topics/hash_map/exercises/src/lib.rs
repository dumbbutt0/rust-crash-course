use std::collections::HashMap;

pub fn init(address: String, amount: u32) -> HashMap<String, u32> {
    let mut s: HashMap<String,u32> = HashMap::new();
    s.insert(address,amount);
    s
}
