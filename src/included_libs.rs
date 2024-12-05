const LIBS: [(&str, &[u8]); 1] = [
    ("std.seal", include_bytes!("../libs/std.seal")),
];

pub fn libs() -> Vec<(String, String)> {
    let mut libs = Vec::new();
    for (name, lib) in LIBS.iter() {
        libs.push((name.to_string(), String::from_utf8_lossy(lib).to_string()));
    }
    libs
}
