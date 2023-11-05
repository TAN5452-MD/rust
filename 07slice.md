## 字符串slice
语法:
```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}
```

对于rust的..range语法 如果想要从索引0开始，可以不写2个点号之间的值
let silce = &s[..2];
同样的，如果想要到结尾，可以不写2个点号之间的值

也可以同时舍弃开始和结尾的值来获取整个字符串
let silce = &s[..];

```rust
let s = String::from("hello");//需要&引用
let my_string_literal = "hello world"; //直接就是一个slice引用
```

//将String转化为字节数组
let bytes = s.as_bytes();


