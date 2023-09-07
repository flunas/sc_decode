//! # sc_decode
//! 
//! `sc_decode` 是一个对特殊字符进行解码的小工具。
//! 
//! 有个别特殊字符解码后使用format!格式化后会有 `\` ，需自行去除。 
/// 对特殊字符进行解码
/// 
/// # Examples
/// ```
/// let arg = "123%40gmail.com";
/// let val = sc_decode::decode(arg);
/// 
/// assert_eq!("123@gmail.com", val);
/// ```
pub fn decode(value: &str) -> String {
    let mut val:String = value.to_string();
    let index_arr:Vec<_> = value.rmatch_indices("%").collect();
    for (v, _s) in index_arr {
        match value.get(v..v+3) {
            Some("%2C") => val = val.replace("%2C", ","),
            Some("%3A") => val = val.replace("%3A", ":"),
            Some("%27") => val = val.replace("%27", "'"),
            Some("%22") => val = val.replace("%22", "\""),
            Some("%3F") => val = val.replace("%3F", "?"),
            Some("%2F") => val = val.replace("%2F", "/"),
            Some("%5C") => val = val.replace("%5C", "\\"),
            Some("%7C") => val = val.replace("%7C", "|"),
            Some("%3B") => val = val.replace("%3B", ";"),
            Some("%5B") => val = val.replace("%5B", "["),
            Some("%5D") => val = val.replace("%5D", "]"),
            Some("%7B") => val = val.replace("%7B", "{"),
            Some("%7D") => val = val.replace("%7D", "}"),
            Some("%3D") => val = val.replace("%3D", "="),
            Some("%2B") => val = val.replace("%2B", "+"),
            Some("%60") => val = val.replace("%60", "`"),
            Some("%21") => val = val.replace("%21", "!"),
            Some("%40") => val = val.replace("%40", "@"),
            Some("%23") => val = val.replace("%23", "#"),
            Some("%24") => val = val.replace("%24", "$"),
            Some("%25") => val = val.replace("%25", "%"),
            Some("%5E") => val = val.replace("%5E", "^"),
            Some("%26") => val = val.replace("%26", "&"),
            Some("%2A") => val = val.replace("%2A", "*"),
            Some("%28") => val = val.replace("%28", "("),
            Some("%29") => val = val.replace("%29", ")"),
            Some("%3C") => val = val.replace("%3C", "<"),
            Some("%3E") => val = val.replace("%3E", ">"),
            _ => (),
        }
    }

    val
}
