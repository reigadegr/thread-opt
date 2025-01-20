#!/bin/python3
import os


def task():
    os.system("ruff format make.py")
    os.system("ruff format maketools")
    os.system("shfmt -l -s -w -p $(shfmt -f module)")
    os.system("cargo fmt -v")
