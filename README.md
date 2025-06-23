<!--
SPDX-FileCopyrightText: 2025 Chen Linxuan <me@black-desk.cn>

SPDX-License-Identifier: MIT
-->

# up2date

en | [zh_CN](README.zh_CN.md)

> [!WARNING]
> This English README is translated from the Chinese version
> using AI and may contain errors.

A command-line tool to check if all dependencies in the current repository have been configured for automatic updates via dependabot.

## Usage

```bash
up2date # Markdown output
up2date --json # JSON output
up2date --yaml # YAML output
up2date --toml # TOML output
```

## License

Unless otherwise specified, the code of this project are open source under the
GNU General Public License version 3 or any later version, while documentation,
configuration files, and scripts used in the development and maintenance process
are open source under the MIT License.

This project complies with the [REUSE specification].

You can use [reuse-tool](https://github.com/fsfe/reuse-tool) to generate the
SPDX list for this project:

```bash
reuse spdx
```

[REUSE specification]: https://reuse.software/spec-3.3/
