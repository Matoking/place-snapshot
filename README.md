place-snapshot
==============

Simple command-line application to parse the [r/Place](https://reddit.com/r/place/) GZIP
compressed dataset and render the canvas at given time into an PNG file.
The dataset is a GZIP compressed CSV file with 160 million rows.

This repository contains two implementations: one written in Rust and another
written in Python.

# Usage

## Preparation

To actually do anything with this application, you'll need to download the r/Place
GZIP compressed dataset, which you can find [here](https://reddit.com/r/place/comments/txvk2d/rplace_datasets_april_fools_2022/).

If you're in a hurry, you can download it using `wget`:

```
$ wget https://placedata.reddit.com/data/canvas-history/2022_place_canvas_history.csv.gzip
```

Both implementations accept roughly the same command-line parameters and place (hehe)
the finished image file `place.png` into the working directory.

## Rust

Install Cargo and Rust toolchain, then run the following command to build and render the PNG file:

```
# Remember '-r' flag in order to build in release mode
$ cargo run -r -- --path 2022_place_canvas_history.csv.gzip "2022-04-05 15:00:00"
```

You can also find the binary under `target/release`.

## Python

Install Python 3, then create a virtualenv and install requirements as follows:

```
python3 -mvenv venv
source venv/bin/activate  # Activate virtualenv
pip install --upgrade pip setuptools  # Just in case your distro has old pip and setuptools
pip install -r requirements.txt
python place-snapshot.py 2022_place_canvas_history.csv.gzip "2022-04-05 15:00:00"
```

# Motivation

This was a toy program to test writing a simple but practical application in
both Rust and Python and measure their performance after optimizing both of
them. The project also allowed to measure the effort required to write a
non-trivial program in Rust.

Author had little prior experience in Rust and far more experience in writing Python.

# Results

After optimizing both programs to some degree, the Rust implementation is nearly
5x as fast as the Python implementation.

```
# Tests performed on a Ryzen 5900X processor
# Run Rust implementation three times
# (rustc 1.64.0)
$ perf stat -r 3 ./place-snapshot --path "/home/matoking/2022_place_canvas_history.csv.gzip" "2022-04-04 18:00:00"

...

            99.725 +- 0.112 seconds time elapsed  ( +-  0.11% )

# Run Python implementation three times
# (Python 3.10.8)
$ perf stat -r 3 python place-snapshot.py "/home/matoking/2022_place_canvas_history.csv.gzip" "2022-04-04 18:00:00"

            490.65 +- 1.22 seconds time elapsed  ( +-  0.25% )
...

```

The Rust implementation was fairly easy to write with basic Rust knowledge.
Both the Clippy linter and the compiler provide helpful hints when faced with
erroneous Rust code. Many mistakes could be fixed by replacing the problematic
code with the one suggested by the compiler.

More complex programs might be harder to implement when dealing with more complicated
program architecture. The program is single threaded and processes
a single file in serial, an approach that gels nicely with Rust's ownership model.
