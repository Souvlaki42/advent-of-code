# Advent of Code

My advent of code solutions starting from 2024. I'm using rust for now. I might use other languages or try other years' challenges in the future.

> [!NOTE]
> This is my personal repo for my solutions to the advent of code challenges.
> I don't include my input files in this repo, so you should use your own.
> I'm not trying to compete with anyone, I just want to learn and have fun solving some challenges.

> [!IMPORTANT]
> This is not neccessarily the most optimal solution. I might do some optimizations later on.

> [!WARNING]
> I'm not responsible for any consequences of using this repo. Use it at your own risk.

## Dependencies

- Install [Rust 1.89+](https://www.rust-lang.org/tools/install)
- Install [Python 3.10+](https://www.python.org/downloads/)
- Install [AoC CLI](https://github.com/scarvalhojr/aoc-cli/)
- (Optional) Install [Cargo Generate](https://github.com/cargo-generate/cargo-generate)

## Usage

First, make a `.env` file in the root of the project with the following content:

```dotenv
# Your AoC session (https://github.com/scarvalhojr/aoc-cli/?tab=readme-ov-file#session-cookie-)
ADVENT_OF_CODE_SESSION=<your-seesion-cookie>
```

Then, install python dependencies:

```bash
pip install -r requirements.txt
```

- Run: `python3 manager.py run -y <year> -d <day> -p <part>`
- Generate: `python3 manager.py gen -y <year> -d <day>`
- Download: `python3 manager.py download -y <year> -d <day>`
- Submit: `python3 manager.py submit -y <year> -d <day> -p <part> -a <answer>`

## License

[Apache 2.0](LICENSE)

