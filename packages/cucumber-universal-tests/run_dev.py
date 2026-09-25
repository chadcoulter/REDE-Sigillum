#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import subprocess
import sys


PACKAGE_ROOT = Path(__file__).resolve().parent
CONFIG_PATH = PACKAGE_ROOT / "cucumber.dev.json"


def load_config() -> dict:
    with CONFIG_PATH.open("r", encoding="utf-8") as handle:
        config = json.load(handle)

    if config.get("defaultEnabled") is not True:
        raise ValueError(
            "cucumber.dev.json must keep defaultEnabled=true; "
            "development overrides may disable individual tags."
        )

    overrides = config.get("tagOverrides", {})
    if not isinstance(overrides, dict):
        raise ValueError("tagOverrides must be a JSON object.")

    for tag, enabled in overrides.items():
        if not isinstance(tag, str) or not tag.startswith("@"):
            raise ValueError(f"Invalid Cucumber tag: {tag!r}")
        if not isinstance(enabled, bool):
            raise ValueError(f"Override for {tag} must be true or false.")

    return config


def development_tag_expression(config: dict) -> str | None:
    disabled = sorted(
        tag
        for tag, enabled in config["tagOverrides"].items()
        if enabled is False
    )

    if not disabled:
        return None

    return " and ".join(f"not {tag}" for tag in disabled)


def runner_command(runner: str, tag_expression: str | None) -> tuple[list[str], Path, dict]:
    env = os.environ.copy()

    if runner == "rust":
        command = ["cargo", "test", "--test", "cucumber", "--"]
        cwd = PACKAGE_ROOT / "runners" / "rust"
        if tag_expression:
            command.extend(["--tags", tag_expression])
        return command, cwd, env

    if runner == "python":
        command = [sys.executable, "-m", "behave"]
        cwd = PACKAGE_ROOT / "runners" / "python"
        if tag_expression:
            command.extend(["--tags", tag_expression])
        return command, cwd, env

    if runner == "ruby":
        command = ["bundle", "exec", "cucumber"]
        cwd = PACKAGE_ROOT / "runners" / "ruby"
        env["BUNDLE_GEMFILE"] = str(cwd / "Gemfile")
        if tag_expression:
            command.extend(["--tags", tag_expression])
        return command, cwd, env

    raise ValueError(f"Unsupported runner: {runner}")


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Run REDE-Sigillum shared Cucumber tests using development tag overrides."
    )
    parser.add_argument("runner", choices=("rust", "python", "ruby"))
    args = parser.parse_args()

    config = load_config()
    expression = development_tag_expression(config)

    command, cwd, env = runner_command(args.runner, expression)

    if expression:
        print(f"Development tag filter: {expression}")
    else:
        print("Development tag filter: all shared tests enabled")

    completed = subprocess.run(command, cwd=cwd, env=env, check=False)
    return completed.returncode


if __name__ == "__main__":
    raise SystemExit(main())
