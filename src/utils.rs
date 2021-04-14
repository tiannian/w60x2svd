use crate::mode::AccessMode;

pub fn from_radix_to_u32(s: &str) -> Option<u32> {
    let s_trim = s.trim();
    if s_trim != "" {
        let remove_str = s_trim.replace("_", "");
        // println!("{}", remove_str);
        let r = u32::from_str_radix(&remove_str[2..], 16).unwrap();
        Some(r)
    } else {
        None
    }
}

pub fn from_string_to_access(s: &str) -> Option<AccessMode> {
    let mode = match s {
        "RW" => Some(AccessMode::RW),
        "RO" => Some(AccessMode::RO),
        "WO" => Some(AccessMode::RW),
        _ => None,
    };
    mode
}
