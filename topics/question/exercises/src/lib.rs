fn parse(s: &str) -> Result<u32, String> {
    match s.parse() {
        Ok(val) => Ok(val),
        Err(_) => Err("Failed to parse string into u32".to_string()),
    }
}

pub fn sum(nums: &[&str]) -> Result<u32, String> {
    let mut z = 0u32;
    for &i in nums{
        let val = parse(i)?;
        z+=val;
    }
    Ok(z)
}
