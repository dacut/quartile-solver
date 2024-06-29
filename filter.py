#!/usr/bin/env python3

rejected = set()

with open("words-reject.txt", "r") as ifd:
    for line in ifd:
        line = line.strip()
        if line:
            rejected.add(line)

with open("words-orig.txt", "r") as ifd:
    with open("words.txt", "w") as ofd:
        for line in ifd:
            word = line.strip().lower()
            if word.isalpha() and len(word) > 1 and word not in rejected:
                ofd.write(word.lower() + "\n")
