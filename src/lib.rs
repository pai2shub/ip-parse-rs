/*
请使用Rust语言实现IPv4地址转换为整数，例如
"172.168.5.1" => 2896692481
要求
1.只能遍历一次字符串
2.若 . 附近存在空格，为有效输入；若数字中存在空格，则报错.
    例如:
        “172.168.5.1” 为有效输入，正常应答
        "1 72.168.5.1"为无效输入，返回错误
3.提供完善的单元测试
*/
pub fn ipv4_to_int(ip: &str) -> Result<u32, &'static str> {
    let mut result = 0;
    let mut num = 0;
    let mut count = 0;
    for c in ip.chars() {
        if c == '.' {
            if count == 0 {
                return Err("Invalid input");
            }
            result = result << 8 | num;
            num = 0;
            count = 0;
        } else if c == ' ' {
            return Err("Invalid input");
        } else if c.is_digit(10) {
            num = num * 10 + c.to_digit(10).unwrap();
            count += 1;
        } else {
            return Err("Invalid input");
        }
    }
    if count == 0 {
        return Err("Invalid input");
    }
    result = result << 8 | num;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_to_int() {
        assert_eq!(ipv4_to_int("172.168.5.1"), Ok(2896692481));
        assert_eq!(ipv4_to_int("1 72.168.5.1"), Err("Invalid input"));
        assert_eq!(ipv4_to_int("172.168.5.1 "), Err("Invalid input"));
    }
}
