#!/bin/python3
import os
from maketools.toolchains import Buildtools


def task():
    tools = Buildtools()

    os.system("ruff check --fix make.py")
    os.system("ruff check --fix maketools")

    (
        tools.cargo()
        .arg("clippy --fix --allow-dirty --allow-staged --target aarch64-linux-android")
        .build()
    )

    (
        tools.cargo()
        .arg(
            "clippy --fix --allow-dirty --allow-staged --target aarch64-linux-android --release"
        )
        .build()
    )
