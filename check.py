#!/usr/bin/env python3

from os import listdir, system

for file in listdir("sample/"):
    if "input" not in file:
        continue

    i = file.strip("input").strip(".csv")
    system(f"cargo run -- sample/{file} > sample/myoutput{i}.csv")
    system(f"diff <(sort sample/output{i}.csv) <(sort sample/myoutput{i}.csv)")