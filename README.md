# Advent of Code

My AoC solutions starting from 2024. I'm using Rust for it but I use Python for my CLI automations.

> [!NOTE]
> This is my personal repo for my solutions to the advent of code challenges.
> I don't include my input files in this repo, so you should use your own.
> I'm not trying to compete with anyone, I just want to learn and have fun solving some challenges.

> [!IMPORTANT]
> This is not neccessarily the most optimal solution. I might do some optimizations later on.

> [!WARNING]
> I'm not responsible for any consequences of using this repo. Use it at your own risk.

## Dependencies

- Install [Rust 1.89+](https://www.rust-lang.org/tools/install/)
- Install [UV](https://docs.astral.sh/uv/getting-started/installation/)
- Install [AoC CLI](https://github.com/scarvalhojr/aoc-cli/)
- Install [Cargo Generate](https://github.com/cargo-generate/cargo-generate/)

## Usage

First, make sure to store your AoC session cookie inside `~/.adventofcode.session` file, as per the [AoC CLI](https://github.com/scarvalhojr/aoc-cli/?tab=readme-ov-file#session-cookie-) instructions.

Then, setup your Python virtual environment and install the dependencies:

```bash
uv venv
source venv/bin/activate
venv sync
```

To leave the virtual environment, run `deactivate`.

While inside the virtual environment, you can run the following commands:

- Run: `aoc-manager run -y <year> -d <day> -p <part>`
- Generate: `aoc-manager gen -y <year> -d <day>`
- Download input file: `aoc-manager download -y <year> -d <day>`
- Submit solution: `aoc-manager submit -y <year> -d <day> -p <part> -s <solution>`

The `day` argument can be a single number or a range of numbers, e.g. `1..15` or `1,3,7..10`.

## License

This project is released into the public domain. See the [UNLICENSE](UNLICENSE) file for more information.

