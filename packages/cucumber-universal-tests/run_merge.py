#!/usr/bin/env python3
from __future__ import annotations

import argparse
import os
from pathlib import Path
import subprocess
import sys


PACKAGE_ROOT = Path(__file__).resolve().parent


def runner_command(runner: str) -> tuple[list[str], Path, dict]:
    env = os.environ.copy()

    if runner == "rust":
        return (
            ["cargo", "test", "--test", "cucumber", "--"],
            PACKAGE_ROOT / "runners" / "rust",
            env,
        )

    if runner == "python":
        return (
            [sys.executable, "-m", "behave"],
            PACKAGE_ROOT / "runners" / "python",
            env,
        )

    if runner == "ruby":
        cwd = PACKAGE_ROOT / "runners" / "ruby"
        env["BUNDLE_GEMFILE"] = str(cwd / "Gemfile")
        return (
            ["bundle", "exec", "cucumber"],
            cwd,
            env,
        )

    raise ValueError(f"Unsupported runner: {runner}")


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Run every REDE-Sigillum shared Cucumber scenario for merge validation."
    )
    parser.add_argument("runner", choices=("rust", "python", "ruby"))
    args = parser.parse_args()

    command, cwd, env = runner_command(args.runner)

    print("Merge mode: running all shared scenarios; cucumber.dev.json is ignored")

    completed = subprocess.run(command, cwd=cwd, env=env, check=False)
    return completed.returncode


if __name__ == "__main__":
    raise SystemExit(main())
