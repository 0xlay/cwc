# CWC - Crossplatform Word Count utility

The cwc is a cross-platform implementation of the `wc` (word count) Unix-like utility, designed to work on Windows, macOS, and Linux systems.

## Overview

`cwc` provides a convenient way to count bytes, words, lines, and characters in a given input file. It aims to offer a consistent experience across different operating systems, ensuring reliable functionality regardless of the platform.

## Options
- `-c` or `--bytes` - display the byte count.
- `-m` or `--chars` - display the character count.
- `-l` or `--lines` - display the newline count.
- `-w` or `--words` - display the word count.

## How to build
```cargo build```