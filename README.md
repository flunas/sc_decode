# sc_decode
`sc_decode` 是一个对特殊字符进行解码的小工具。

有个别特殊字符解码后使用format!格式化后会有 `\` ，需自行去除。 

## Method

decode - 对特殊字符进行解码

## Examples

```
let arg = "123%40gmail.com";
let val = sc_decode::decode(arg);

assert_eq("123@gmail.com", val);