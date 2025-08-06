pub fn hello() -> String {
  let s =  String::from("Hello Rust");
  s
}

pub fn greet(name: &str) -> String {
    let s = format!("Hello {name}");
    s.to_string()
}

pub fn append(mut s: String) -> String {
    s += "!";
    s
}
