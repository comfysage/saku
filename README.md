# saku

:seedling: a tiny distro-independent package manager written in Rust.

![saku-show](./assets/show-neovim.gif)

# Saku

Saku is a lightweight package manager for macOS/linux, written in Rust. It aims to
provide an alternative solution to managing software packages on macOS/linux systems,
addressing some of the issues faced with existing package managers like
Homebrew.

## Features

- Simplified package management: Saku strives to provide a streamlined and
  user-friendly package management experience, making it easy to install,
  update, and remove software packages.

- Lightweight and efficient: Saku is designed to be lightweight and
  efficient, minimizing resource usage while maintaining performance. It aims
  to provide fast and responsive package management operations.

- Improved stability: Saku focuses on stability and reliability, aiming to
  minimize dependency conflicts and provide a robust package installation
  process.

- Self maintaining: Saku has built-in support for installing updates and
  configuring new packages.

## Installation

Run the installer using curl:

```shell
curl -fsSL https://raw.githubusercontent.com/CrispyBaccoon/saku/mega/install.sh | sh
```

Setup saku environment, add this to your `.bashrc`/`.zshrc`:

```shell
eval "$($HOME/.saku/root/bin/saku env)"
```

## Usage

Saku provides a simple command-line interface for managing packages. Here
are some common commands:

- `saku install <package>`: Installs the specified package.
- `saku update`: Update sources.
- `saku upgrade <package>`: Updates the specified package to the latest version.
- `saku remove <package>`: Removes the specified package.

For more information on available commands and options, refer to the Saku
documentation.

## Contributing

Contributions to Saku are welcome! If you encounter any issues, have
suggestions for improvements, or want to contribute new features, please submit
a pull request or open an issue on the GitHub repository.

Before contributing, please review the [contribution
guidelines](https://github.com/crispybaccoon/saku/blob/main/CONTRIBUTING.md)
for instructions on how to contribute code, report bugs, and more.

## License

Saku is released under the MIT License. Please review the license file for more details.

## Acknowledgements

Saku is inspired by package managers like Homebrew. Special thanks to the
contributors and the open-source community for their valuable contributions.

## Contact

For any inquiries or questions, feel free to contact the project maintainer at
[[67917529+CrispyBaccoon@users.noreply.github.com]].
