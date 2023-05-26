#!/usr/bin/env python3

import subprocess
import sys
import os

args = sys.argv[1:]

# Make sure args is non-empty
if not args:
    print("Error: no arguments provided for rustc")
    sys.exit(1)

# Call rustc with the arguments
try:
    rustc_args = ["rustc"] + args
    subprocess.run(rustc_args, check=True)
except subprocess.CalledProcessError as e:
    sys.exit(e.returncode)

# Call valgrind if this is the actual rustc test run invocation
should_run_valgrind = os.getenv("UITEST_TEST_RUN", "False") == "True"
if should_run_valgrind:
    # Call rustc again to get the binary's filename
    rustc_args = ["rustc", "--print", "file-names"] + args
    result = subprocess.run(rustc_args, stdout=subprocess.PIPE)
    filename = result.stdout.decode().strip()

    # Get the output directory so that we can have the full path to the binary
    if "--out-dir" in args:
        out_dir_index = args.index("--out-dir")
        out_dir_path = args[out_dir_index + 1]
        full_path = out_dir_path + "/" + filename
        args[out_dir_index + 1] = full_path

    valgrind_args = ["../bin/valgrind", "-q", "--tool=krabcake", "--normalize-output=yes", full_path]
    subprocess.run(valgrind_args)
