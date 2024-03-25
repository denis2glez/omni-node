## 🔎 Content

- [🔎 Content](#-content)
- [🤔 About ](#-about-)
- [🏁 Getting Started ](#-getting-started-)
- [🔧 Development ](#-development-)
  - [Quick check ](#quick-check-)
  - [Build ](#build-)
  - [Run tests ](#run-tests-)
- [⚙️ Run the application ](#️-run-the-application-)
- [🚀 Deployment ](#-deployment-)
- [🎉 Acknowledgements ](#-acknowledgements-)
- [📝 License ](#-license-)

## 🤔 About <a name = "about"></a>

This repository implements a networked application comprising a single binary called `omni-node`
capable of functioning as either a server or a client.

## 🏁 Getting Started <a name = "getting-started"></a>

To use your host system as development environment install the following prerequisites.

- `curl`, `git`
- [Rust](https://www.rust-lang.org/tools/install)

> [!TIP]
> In any case, you can check below for suggestions on how to install the prerequisites on your system.

<details open>
<summary><b>Linux (Debian/Ubuntu)</b></summary>

If you are using Debian or a derivative (e.g. Ubuntu, Linux Mint), it is recommended to install Rust
using the standard installation script. You could install all the development prerequisites by running
the following commands.
```sh
sudo apt install curl git
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
</details>

<details close>
<summary><b>macOS</b></summary>

If you are using macOS you could install all the development prerequisites using [Homebrew](https://brew.sh)
by running the following commands.
```sh
brew install curl git
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
</details>

<details close>
<summary><b>Windows</b></summary>

If you are using Windows, you could install all the development prerequisites using the
[`winget`](https://docs.microsoft.com/en-us/windows/package-manager/winget/#production-recommended)
CLI tool by running the following commands.
```sh
winget install --id Git.Git
winget install --id Rustlang.Rust.MSVC
```
</details>

## 🔧 Development <a name = "development"></a>

Once you have a development environment configured with all the necessary prerequisites, you can
perform any of the following tasks.

### Quick check <a name = "quick-check"></a>

Quickly check the package and all of its dependencies for possible errors
```sh
cargo check
```

### Build <a name = "build"></a>

To build the packages use
```sh
cargo build
```

### Run tests <a name = "run-tests"></a>

Now we can run all the default tests
```sh
cargo test
```
or just a specific group of tests, by adding `-- <pattern>` to filter.

## ⚙️ Run the application <a name = "-run-the-application"></a>

To run the networked application, let's start by running the `omni-node` binary in server mode
```sh
cargo run --release -- --mode server
```
and from here let's execute as many processes in client mode as we want by
```sh
cargo run --release
```

> [!TIP]
> To check out all the options available on the command line use `cargo run --release -- --help`.

## 🚀 Deployment <a name = "deployment"></a>

You can use any of the tarballs in the [Releases section](https://github.com/denis2glez/omni-node/releases)
to deploy the software according to your requirements. These are automatically generated using the
release workflow after tagging a new version.

## 🎉 Acknowledgements <a name = "acknowledgement"></a>

Thanks to all the developers of the libraries used throughout the project.

## 📝 License <a name = "license"></a>

This project is licensed under the [MIT](LICENSE) license.