#!/bin/python3
import sys
import maketools.format_codes as format_codes
import maketools.build as build
import maketools.fix as fix
import maketools.update as update
from maketools.misc import eprint

help_text = """\
./make.py:
    build:
        build and package fas-rs module
        sugg: try ./make.sh build --help to get details
    format:
        format codes of fas-rs
    fix:
        fix codes of fas-rs
    update:
        recursive update all depended crates
    help:
        print this help\
"""

try:
    arg = sys.argv[1]
except IndexError:
    eprint("Missing argument")
    eprint(help_text)
    exit(-1)

match arg:
    case "help":
        print(help_text)
    case "fmt" | "format":
        format_codes.task()
    case "build":
        build.task(sys.argv[2:])
    case "fix":
        fix.task()
    case "update":
        update.task()
    case _:
        eprint("Invalid argument")
        eprint(help_text)
        exit(-1)
