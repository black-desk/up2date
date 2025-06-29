<!--
SPDX-FileCopyrightText: 2025 Chen Linxuan <me@black-desk.cn>

SPDX-License-Identifier: MIT
-->

# up2date

[![checks][badge-shields-io-checks]][actions]
[![commit activity][badge-shields-io-commit-activity]][commits]
[![contributors][badge-shields-io-contributors]][contributors]
[![release date][badge-shields-io-release-date]][releases]
![commits since release][badge-shields-io-commits-since-release]
[![codecov][badge-shields-io-codecov]][codecov]

[badge-shields-io-checks]:
  https://img.shields.io/github/check-runs/black-desk/up2date/master
[actions]: https://github.com/black-desk/up2date/actions
[badge-shields-io-commit-activity]:
  https://img.shields.io/github/commit-activity/w/black-desk/up2date/master
[commits]: https://github.com/black-desk/up2date/commits/master
[badge-shields-io-contributors]:
  https://img.shields.io/github/contributors/black-desk/up2date
[contributors]: https://github.com/black-desk/up2date/graphs/contributors
[badge-shields-io-release-date]:
  https://img.shields.io/github/release-date/black-desk/up2date
[releases]: https://github.com/black-desk/up2date/releases
[badge-shields-io-commits-since-release]:
  https://img.shields.io/github/commits-since/black-desk/up2date/latest
[badge-shields-io-codecov]:
  https://codecov.io/github/black-desk/up2date/graph/badge.svg?token=6TSVGQ4L9X
[codecov]: https://codecov.io/github/black-desk/up2date

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
