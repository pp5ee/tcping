<div align="center" style="width: 100%;">
 <img alt="tcping" src="Artwork/tcping_logo3.jpeg" style="width:70%;">
</div>

# TCPING - Rust 实现

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/pp5ee/tcping)](https://github.com/pp5ee/tcping/releases)
[![Ubuntu Package](https://img.shields.io/badge/platform-Ubuntu%20Linux-green.svg)](https://github.com/pp5ee/tcping/releases)
![GitHub downloads](https://img.shields.io/github/downloads/pp5ee/tcping/total.svg?label=DOWNLOADS&logo=github)

这是一个用 Rust 重写的 `TCP` 端口 ping 程序，与原始 Go 版本具有 100% CLI 兼容性。此程序将向您指定的 `IP 地址` 或 `主机名` 发送 `TCP` 探测，并打印结果。它支持 `IPv4` 和 `IPv6`，专为 Ubuntu Linux 优化部署。

**TCPING** 对 _成功_ 和 _不成功_ 的探测使用不同的 `TCP 序列号`，因此当您查看结果并发现探测失败时，可以很容易地推断出到该点为止的总丢包数。

以下是 **TCPING Rust 实现** 的一些功能：

- **100% CLI 兼容** - 与原始 Go 版本完全相同的命令行界面
- **全面统计** - 正常运行时间/停机时间跟踪，RTT 最小/平均/最大值，主机名变更跟踪
- **多种输出格式** - 人类可读、JSON、CSV 和 SQLite 数据库输出
- **实时功能** - 按 Enter 键获取实时统计信息，Ctrl+C 优雅关闭
- **Ubuntu 部署** - 专为 Ubuntu Linux 优化，交叉编译目标为 x86_64-unknown-linux-gnu
- **高级网络功能** - IPv4/IPv6 支持，接口绑定，主机名解析重试
- 在 `ICMP` 被阻止的环境中替代 `ping`。
- 监控您的网络连接。
- 确定丢包率。
- 分析网络延迟。
- 计算网络探测的 `最小`、`平均` 和 `最大` 延迟。
- 按下 `Enter` 键即可打印连接统计信息，而无需停止程序。
- 使用 `-r` 标志在预定次数的探测失败后重试主机名解析。适用于测试您的 `DNS` 负载均衡或全局服务器负载均衡器 `(GSLB)`。
- 强制使用 `IPv4` 或 `IPv6`。
- 显示遇到的最长 `停机时间` 和 `正常运行时间` 持续时间和时间。
- 监控和审计您的对等网络 (SLA)。
- 在进行维护时计算网络的总正常运行时间或停机时间。
- 提供彩色、纯文本、JSON、CSV 和 SQLite3 多种输出格式。

---

## 目录

- [TCPING](#tcping)
  - [目录](#目录)
  - [演示](#演示)
    - [基本用法](#基本用法)
    - [重试主机名查找 (`-r`)](#重试主机名查找--r)
    - [JSON 格式输出 (`-j --pretty`)](#json-格式输出--j---pretty)
  - [下载](#下载)
  - [用法](#用法)
    - [Linux - Debian 和 Ubuntu](#linux---debian-和-ubuntu)
    - [Linux、BSD 和 mac OS](#linuxbsd-和-mac-os)
    - [Windows](#windows)
    - [Docker](#docker)
  - [标志](#标志)
  - [提示](#提示)
  - [检查更新](#检查更新)
  - [贡献](#贡献)
  - [功能请求和问题](#功能请求和问题)
  - [测试平台](#测试平台)
  - [帮助项目](#帮助项目)
  - [许可证](#许可证)

---

## 演示

### 基本用法

![tcping](Images/gifs/tcping.gif)

---

### 重试主机名查找 (`-r`)

![tcping resolve example](Images/gifs/tcping_resolve.gif)

---

### JSON 格式输出 (`-j --pretty`)

![tcping json example](Images/gifs/tcping_json_pretty.gif)

---

## 下载

- ### [Ubuntu 软件包](https://github.com/pp5ee/tcping/releases/latest/download/tcping_*.tar.gz) - 专为 Ubuntu Linux 优化的 tar 包

下载完成后，请转到[用法](#用法)部分。

**或者**，您可以：

- **从源代码构建**：

  ```bash
  # 安装 Rust 工具链
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

  # 克隆并构建
  git clone https://github.com/pp5ee/tcping.git
  cd tcping
  cargo build --release

  # 创建 Ubuntu 软件包
  ./build-ubuntu-tar.sh
  ```

- 使用 `cargo install` 安装：

  ```bash
  cargo install --git https://github.com/pp5ee/tcping.git
  ```

---

## 用法

请按照您操作系统的说明进行操作：

- [TCPING](#tcping)
  - [目录](#目录)
  - [演示](#演示)
    - [基本用法](#基本用法)
    - [重试主机名查找 (`-r`)](#重试主机名查找--r)
    - [JSON 格式输出 (`-j --pretty`)](#json-格式输出--j---pretty)
  - [下载](#下载)
  - [用法](#用法)
    - [Linux - Debian 和 Ubuntu](#linux---debian-和-ubuntu)
    - [Linux、BSD 和 mac OS](#linuxbsd-和-mac-os)
    - [Windows](#windows)
    - [Docker](#docker)
  - [标志](#标志)
  - [提示](#提示)
  - [检查更新](#检查更新)
  - [贡献](#贡献)
  - [功能请求和问题](#功能请求和问题)
  - [测试平台](#测试平台)
  - [帮助项目](#帮助项目)
  - [许可证](#许可证)

另请查看[此处提供的可用标志](#标志)。

### Linux - Debian 和 Ubuntu

在 **Debian** 及其衍生版本（如 **Ubuntu**）上，下载 `.deb` 软件包：

```bash
wget https://github.com/pouriyajamshidi/tcping/releases/latest/download/tcping-amd64.deb -O /tmp/tcping.deb
```

并安装它：

```bash
sudo apt install -y /tmp/tcping.deb
```

如果您使用的是其他 Linux 发行版，请继续阅读[本节](#linux-bsd-和-mac-os)。

### Ubuntu 安装

下载并解压缩 Ubuntu 软件包：

```bash
# 下载并解压缩 Ubuntu 软件包
wget https://github.com/[username]/tcping/releases/latest/download/tcping_2.7.1_x86_64.tar.gz
tar -xzf tcping_2.7.1_x86_64.tar.gz

# 系统级安装
cd tcping_2.7.1_x86_64
sudo ./install.sh
```

运行：

```bash
tcping www.example.com 443
# 或
tcping 10.10.10.1 22
```

您还可以使用 `主机:端口` 格式：

```bash
tcping www.example.com:443
# 或使用 IP 地址
tcping 192.168.1.1:80
# IPv6 地址（使用引号防止 shell 解析）
tcping '[2001:db8::1]:443'
```

## 发布自动化

此项目包含 GitHub Actions 自动化功能：
- 标签发布时自动构建 Ubuntu 软件包
- 针对 x86_64-unknown-linux-gnu 目标的交叉编译
- 包含验证的 tar 包创建
- GitHub 发布资源上传

---

## 标志

以下标志可用于控制应用程序的行为：

| 标志                   | 描述                                                                             |
| ---------------------- | ------------------------------------------------------------------------------- |
| `-h`                   | 显示帮助                                                                          |
| `-4`                   | 仅使用 IPv4 地址                                                                  |
| `-6`                   | 仅使用 IPv6 地址                                                                  |
| `-r`                   | 在 `<n>` 次探测失败后重试解析目标主机名。例如，-r 10 表示在 10 次探测失败后重试            |
| `-c`                   | 在 `<n>` 次探测后停止，无论结果如何。默认情况下，不应用限制                              |
| `-t`                   | 等待响应的时间（以秒为单位）。允许使用实数。0 表示无限超时                                |
| `-D`                   | 在探测输出中显示日期和时间。类似于 Linux 的 ping 工具，但更易于阅读                      |
| `-i`                   | 发送探测之间的间隔                                                                 |
| `-I`                   | 用于发送探测的接口名称                                                              |
| `--no-color`           | 输出不带颜色                                                                      |
| `--csv`                | 以 CSV 格式输出到指定的文件路径                                                     |
| `-j`                   | 以 `JSON` 格式输出                                                                |
| `--pretty`             | 美化 `JSON` 输出                                                                 |
| `--db`                 | 用于存储 tcping 输出到 sqlite 数据库的路径和文件名。例如 `--db /tmp/tcping.db`         |
| `-v`                   | 打印版本                                                                         |
| `-u`                   | 检查更新                                                                         |
| `--show-failures-only` | 仅显示探测失败，并省略打印探测成功消息                                                |
| `--show-source-address` | 显示探测所用的来源IP地址及端口                                                      | 

> 如果未指定 `-4` 和 `-6` 标志，tcping 将根据 DNS 查找随机选择一个 IP 地址。

---

## 提示

- 在程序运行时按 `Enter` 键，可以在不终止程序的情况下查看所有探测的摘要，如[演示](#演示)部分所示。

---

## 检查更新

`TCPING` 正在不断改进，添加了许多新功能并修复了错误。请务必查看更新的版本。

```bash
tcping -u
```

## 贡献

欢迎提交拉取请求以解决错误、添加新功能以及帮助解决可以在[此处](https://github.com/pouriyajamshidi/tcping/issues)找到的未解决问题

1. 选择您觉得可以处理的任何问题。
1. Fork 存储库。
1. 创建一个分支。
1. 提交您的工作。
1. 如果可能，请添加测试。
1. 运行测试 `go test` 或 `make test` 并确保它们成功。
1. 创建一个拉取请求

当前未解决问题的数量：![GitHub issues](https://img.shields.io/github/issues/pouriyajamshidi/tcping.svg)。

请确保您的拉取请求**仅涵盖一个特定的问题/功能**，并且不处理两个或多个问题。这使我们更容易检查您的拉取请求，并有助于保持干净的 git 历史记录。

## 功能请求和问题

如果您需要新功能或发现错误，请随时[打开拉取请求](#贡献)或提交问题。

> 对于较大的功能/贡献，请确保在开始工作之前先在 `issue` 上进行沟通。

## 测试平台

Windows、Linux 和 macOS。

## 帮助项目

如果 tcping 对您有用，请考虑给它一个 ⭐ 以扩大其影响力并帮助其他人也从中受益。

此外，您可以使用以下链接支持该项目。

请我喝杯咖啡：[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/pouriyajamshidi)

GitHub 赞助：[![sponsor](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86)](https://github.com/sponsors/pouriyajamshidi)

赞助总数：![GitHub Sponsor](https://img.shields.io/github/sponsors/pouriyajamshidi?label=Sponsor&logo=GitHub)

## 许可证

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
