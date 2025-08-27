# Hi

嗨！好久不见~

## 配置文件示例

#### config.toml

```toml
[general]
listen = "0.0.0.0:8000"

[logger]
level = "debug"             # trace, debug, info, warn, error
writer = "file"             # file, stdout
directory = "./log"
file_name_prefix = "hi.log"

```

## 许可证

本项目采用 MIT/Apache-2.0 双重授权模式（可任选其一遵循）：

- [MIT 许可证](LICENSE-MIT)
- [Apache 2.0 版许可证](LICENSE-APACHE)

完整法律声明请查阅对应许可证文件。
