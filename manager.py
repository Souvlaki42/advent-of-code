#! /usr/bin/env python3

import shutil
import sys
from pathlib import Path
from argparse import ArgumentParser, Namespace as Arguments
from pathlib import Path
from subprocess import run
from dotenv import load_dotenv


class CommandError(Exception):
    def __init__(self, cmd: str) -> None:
        self.cmd = cmd
        super().__init__(f"{cmd} is not installed. Please install it.")


def pad2(n: int) -> str:
    return f"{n:02d}"


def run_cmd(args: Arguments) -> str | None:
    year: str = args.year
    day: str = pad2(args.day)
    part: str = args.part
    mp: Path = Path(f"./{year}/day{day}/Cargo.toml")
    if not mp.exists():
        return f"No Cargo.toml at {mp}"
    run(
        f"cargo run --release --manifest-path {mp} --bin part{part}", shell=True)


def gen_cmd(args: Arguments) -> str | None:
    if not shutil.which("cargo-generate"):
        raise CommandError("cargo-generate")

    year: str = args.year
    day: str = pad2(args.day)
    name: str = f"day{day}"
    update = Path(
        f"./{year}/day{args.day}").exists()

    cargo_generate: str = f"cargo generate --path ./template --name {name} --define day={day} --define year={year} --vcs none"
    run(cargo_generate, shell=True)

    mkdir: str = f"mkdir -p \"./{year}\""
    run(mkdir, shell=True)

    if update:
        print(f"Updating {year}/{day}...")
        mv: str = f"mv ./{year}/day{day}/src ./temp/src"
        run(mv, shell=True)

    mv: str = f"mv {name} \"./{year}/{name}\""
    run(mv, shell=True)

    if update:
        mv: str = f"mv ./temp/src ./{year}/day{day}/src"
        run(mv, shell=True)

        rm: str = f"rm -rf ./temp"
        run(rm, shell=True)


def download_cmd(args: Arguments) -> str | None:
    if not shutil.which("aoc"):
        raise CommandError("aoc")

    year: str = args.year
    day: str = pad2(args.day)
    dir: Path = Path(f"./{year}/day{day}")

    mkdir: str = f"mkdir -p {dir}"
    run(mkdir, shell=True)

    aoc_download: str = f"aoc download --year {year} --day {day} --input-only --input-file {dir}/input.txt"
    run(aoc_download, shell=True)


def submit_cmd(args: Arguments) -> str | None:
    if not shutil.which("aoc"):
        raise CommandError("aoc")

    year: str = args.year
    day: str = pad2(args.day)
    part: str = args.part
    answer: str = args.answer

    aoc_submit: str = f"aoc submit {part} \"{answer}\" --year {year} --day {day}"
    run(aoc_submit, shell=True)


def init_patser() -> ArgumentParser:
    yd_parser = ArgumentParser(add_help=False)
    yd_parser.add_argument("-y", "--year", type=int,
                           required=True, help="The year")
    yd_parser.add_argument("-d", "--day", type=int,
                           required=True, help="The day")

    p_parser = ArgumentParser(add_help=False)
    p_parser.add_argument("-p", "--part", type=int,
                          required=True, help="The part number (1 or 2)")

    parser = ArgumentParser(
        description="Manage Advent of Code", prog="manager")
    subparsers = parser.add_subparsers(
        help="The commands available to the manager")

    run_parser = subparsers.add_parser("run", parents=[yd_parser, p_parser])
    run_parser.set_defaults(func=run_cmd)

    gen_parser = subparsers.add_parser("gen", parents=[yd_parser])
    gen_parser.set_defaults(func=gen_cmd)

    download_parser = subparsers.add_parser("download", parents=[yd_parser])
    download_parser.set_defaults(func=download_cmd)

    submit_parser = subparsers.add_parser(
        "submit", parents=[yd_parser, p_parser])
    submit_parser.add_argument(
        "-a", "--answer", type=str, required=True, help="The answer")
    submit_parser.set_defaults(func=submit_cmd)

    return parser


def main():
    load_dotenv()
    parser = init_patser()

    try:
        args = parser.parse_args()
        res = args.func(args)
        if res:
            print("[Error]: ", res)
            sys.exit(1)
    except AttributeError:
        parser.print_help()
    except CommandError as e:
        print("[Command Error]: ", e)
    except Exception as e:
        print("[Exception]: ", e)
        sys.exit(1)

    sys.exit(0)


if __name__ == "__main__":
    main()
