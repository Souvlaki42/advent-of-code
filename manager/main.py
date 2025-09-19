import re
import shutil
import subprocess as sp
from pathlib import Path
import click
from dotenv import load_dotenv


class CommandError(click.ClickException):
    def __init__(self, cmd: str):
        super().__init__(f"{cmd} is not installed. Please install it.")


def year_option() -> click.Option:
    return click.option("-y", "--year", type=int, required=True)


def days_option() -> click.Option:
    return click.option(
        "-d", "--days",
        multiple=True,
        callback=parse_days,
        help="Days: e.g. 1 or 1..15 or 1,3,7..10 (can repeat).",
    )


def part_option() -> click.Option:
    return click.option("-p", "--part", type=int, required=True)


def convert_one_digit_to_two_digits(n: int) -> str:
    return f"{n:02d}"


def parse_days(_ctx, _param, value: tuple[str, ...]) -> list[str]:
    days: set[int] = set()
    pattern = re.compile(r"^\s*\d+\s*(\.\.\s*\d+)?\s*$")
    for token in value:
        for part in token.split(","):
            part = part.strip()
            if not part:
                continue
            if ".." in part:
                if not pattern.match(part):
                    raise click.BadParameter(f"Invalid range syntax: {part}")
                a_s, b_s = [p.strip() for p in part.split("..", 1)]
                a, b = int(a_s), int(b_s)
                if a <= 0 or b <= 0:
                    raise click.BadParameter("Days must be positive integers")
                lo, hi = (a, b) if a <= b else (b, a)
                days.update(range(lo, hi + 1))
            else:
                x = int(part)
                if x <= 0:
                    raise click.BadParameter("Days must be positive integers")
                days.add(x)
    if not days:
        raise click.BadParameter("No days specified")
    res = sorted(days)
    res = [convert_one_digit_to_two_digits(d) for d in res]
    return res


@click.group()
@click.version_option()
def cli():
    """Manage Advent of Code."""
    load_dotenv()


@cli.command()
@year_option()
@days_option()
def gen(year: int, days: list[str]):
    """Generate projects for days."""

    if not shutil.which("cargo-generate"):
        raise CommandError("cargo-generate")

    for day in days:
        name = f"day{day}"
        cmd = (
            f"cargo generate --destination ./{year} --path ./template "
            f"--name {name} --define day={day} --define year={year} --vcs none"
        )
        sp.run(cmd, shell=True, check=True)


@cli.command()
@year_option()
@days_option()
def download(year: int, days: list[str]):
    """Download inputs for days."""

    if not shutil.which("aoc"):
        raise CommandError("aoc")

    for day in days:
        dir_ = Path(f"./{year}/day{day}/src/inputs")
        dir_.mkdir(parents=True, exist_ok=True)
        cmd = f"aoc download --year {year} --day {day} --input-only --input-file {dir_}/input.txt"
        sp.run(cmd, shell=True, check=True)


@cli.command()
@year_option()
@days_option()
@part_option()
def run(year: int, days: list[str], part: int):
    """Run solutions for days."""

    for day in days:
        mp = Path(f"./{year}/day{day}/Cargo.toml")
        if not mp.exists():
            raise click.ClickException(f"No Cargo.toml at {mp}")
        sp.run(
            f"cargo run --release --manifest-path {mp} --bin part{part}", shell=True, check=True)


@cli.command()
@year_option()
@days_option()
@part_option()
@click.option("-s", "--solution", type=str, required=True)
def submit(year: int, days: list[str], part: int, solution: str):
    """Submit answers for days."""

    if not shutil.which("aoc"):
        raise CommandError("aoc")

    for day in days:
        sp.run(
            f"aoc submit {part} "/{solution}/" --year {year} --day {day}", shell=True, check=True)


if __name__ == "__main__":
    cli()
