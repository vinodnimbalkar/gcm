# GD - AI-Powered Git Commit Message Generator

`gd` is a command-line tool that generates meaningful Git commit messages with appropriate emojis based on your Git diff, powered by local LLMs through Ollama.

## Description

This tool pipes your Git diff into an Ollama instance running locally to generate semantically relevant commit messages that follow conventional commit formats with emoji prefixes. It's perfect for developers who want consistent, meaningful commit messages without the mental overhead of crafting them each time.
![GD - AI-Powered Git Commit Message Generator - visual selection](https://github.com/user-attachments/assets/9bdc1145-9b92-4a89-ab40-28b3a3d1c759)


## Features

- Generates commit messages with appropriate emoji prefixes
- Analyzes Git diff content to create contextual commits
- Follows conventional commit standards
- Runs completely locally (no data sent to external APIs)
- Fast and lightweight

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (for building)
- [Ollama](https://ollama.ai/) running locally
- A compatible language model (default: "gemma3")

## Installation

### From Source

```sh
git clone https://github.com/yourusername/gd.git
cd gd
cargo install --path .
```

## Usage

Simply pipe your Git diff to the tool:

```sh
git diff | gd
```

Or for staged changes:

```sh
git diff --staged | gd
```

Then use the generated message to commit:

```sh
git commit -m "$(git diff --staged | gd)"
```

For convenience, you might want to add an alias to your shell configuration:

```sh
# Add to .bashrc, .zshrc, etc.
alias gcm='git commit -m "$(git diff --staged | gd | sed -n '1p')" -m "$(git diff --staged | gd | sed -n '2,$p')"'

```

## Commit Message Format

```
<type>: <emoji><message>
<detailed description>
```

Example:
```
feat: ✨Add user authentication
Implement OAuth2 login flow with Google and GitHub providers
```

## Supported Commit Types and Emojis

| Type                        | Emoji             |
|-----------------------------|-------------------|
| Initial commit              | 🎉 `:tada:`        |
| Version tag                 | 🔖 `:bookmark:`    |
| New feature                 | ✨ `:sparkles:`     |
| Bugfix                      | 🐛 `:bug:`         |
| Metadata                    | 📇 `:card_index:`  |
| Documentation               | 📚 `:books:`       |
| Documenting source code     | 💡 `:bulb:`        |
| Performance                 | 🐎 `:racehorse:`   |
| Cosmetic                    | 💄 `:lipstick:`    |
| Tests                       | 🚨 `:rotating_light:` |
| Adding a test               | ✅ `:white_check_mark:` |
| Make a test pass            | ✔️ `:heavy_check_mark:` |
| General update              | ⚡ `:zap:`          |
| Improve format/structure    | 🎨 `:art:`         |
| Refactor code               | 🔨 `:hammer:`      |
| Removing code/files         | 🔥 `:fire:`        |
| Continuous Integration      | 💚 `:green_heart:` |
| Security                    | 🔒 `:lock:`        |
| Upgrading dependencies      | ⬆️ `:arrow_up:`     |
| Downgrading dependencies    | ⬇️ `:arrow_down:`   |
| Lint                        | 👕 `:shirt:`       |
| Translation                 | 👽 `:alien:`       |
| Text                        | ✏️ `:pencil:`       |
| Critical hotfix             | 🚑 `:ambulance:`   |
| Deploying stuff             | 🚀 `:rocket:`      |
| Fixing on MacOS             | 🍎 `:apple:`       |
| Fixing on Linux             | 🐧 `:penguin:`     |
| Fixing on Windows           | 🏁 `:checkered_flag:` |
| Work in progress            | 🚧 `:construction:` |
| Adding CI build system      | 👷 `:construction_worker:` |
| Analytics or tracking code  | 📈 `:chart_with_upwards_trend:` |
| Removing a dependency       | ➖ `:heavy_minus_sign:` |
| Adding a dependency         | ➕ `:heavy_plus_sign:` |
| Docker                      | 🐳 `:whale:`        |
| Configuration files         | 🔧 `:wrench:`       |
| Package.json in JS          | 📦 `:package:`      |
| Merging branches            | 🔀 `:twisted_rightwards_arrows:` |
| Bad code / needs improvement| 💩 `:hankey:`       |
| Reverting changes           | ⏪ `:rewind:`       |
| Breaking changes            | 💥 `:boom:`         |
| Code review changes         | 👌 `:ok_hand:`      |
| Accessibility               | ♿ `:wheelchair:`   |
| Move/rename repository      | 🚚 `:truck:`        |

## Configuration

The tool currently uses "gemma3" as the default model. You can modify the source code to use different Ollama models.

## License

MIT License

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. Contributions are very welcome!! 😄
