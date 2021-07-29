## ConstantReplacer
用于递归目录下所有文件并替换其中的字符串. 我对rust不熟悉 代码垃圾请见谅...

edit: 根据`Tg:@real186526`曾经对我说的: 能用就是好代码 

∵我的代码能用 ∴我的代码是好代码

[![996.icu](https://img.shields.io/badge/link-996.icu-red.svg)](https://996.icu)
[![LICENSE](https://img.shields.io/badge/license-Anti%20996-blue.svg)](https://github.com/996icu/996.ICU/blob/master/LICENSE)
![RUST VERSION](https://img.shields.io/badge/Rust-2018-blue.svg)

---

### 使用方法:
```rust
// 请原谅我懒了 总之看得懂就好...
println!("需要三个参数: ");
println!(" 1. 需要被替换的文本");
println!(" 2. 将要替换的文本");
println!(" 3. 目标根目录(自动递归子目录)");
println!(" [可选] -q 安静模式, 不输出任何东西");
println!(" [可选] -h 显示这个帮助信息, 不执行任何操作");
println!(" [可选] -v 显示版本信息, 不执行任何操作");
```
---
### 构建方法:

生产环境: `$ cargo build --release`

测试环境: `$ cargo build`

#### 什么? 你问MAKE去哪里了? 没了MAKE我怎么install?

linux install(请先构建): `$ sudo mv ./target/release/ConstantReplacer /usr/bin/cr`

windows install: `> echo "笑死我根本不会写powershell script/bat"`