<!--
SPDX-FileCopyrightText: 2025 Chen Linxuan <me@black-desk.cn>

SPDX-License-Identifier: MIT
-->

# up2date

[en](README.md) | zh_CN

一个用于检查当前仓库中的依赖是否都已经通过dependabot配置了自动更新的命令行工具。

## 使用

```bash
up2date # Markdown输出
up2date --json # JSON输出
up2date --yaml # YAML输出
up2date --toml # TOML输出
```

## 许可证

如无特殊说明，该项目的代码以GNU通用公共许可协议第三版或任何更新的版本开源，文档、配置文件以及开发维护过程中使用的脚本等以MIT许可证开源。

该项目遵守[REUSE规范]。

你可以使用[reuse-tool](https://github.com/fsfe/reuse-tool)生成这个项目的SPDX列表：

```bash
reuse spdx
```

[REUSE规范]: https://reuse.software/spec-3.3/
