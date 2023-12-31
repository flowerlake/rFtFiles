本项目采取以下3种方式进行项目模块的管理

- 通过引入外部第三方源代码lib库，如：ipv6_tool，这种方式在Cargo.toml中添加dependency path即可。
- 本地模块，本地模块统一通过库crate的根文件lib.rs导入，在二进制crate的根文件main.rs中使用libr_fet统一导入到当前crate，其中
  libr_fet在Cargo.toml中进行定义。

```toml
[lib]
name = "libr_fet"
```

- 模块避免使用mod.rs的方式进行导入，使用官方推荐的第一种方式，在main.rs中通过同名文件定义模块，并在其中定义导入的模块、函数和结构体等。