# kc

(See top-level README for instructions on how to use this crate)

Caveats:
- Tests with identical filenames are currently not handled correctly, since they both generate binaries to the same output directory. This seems to be a limitation of `ui_test` itelf, but might be a non-issue for our purposes.

Other notes:
- FIXME(bryangarza): Find a way to suppress Valgrind's non-deterministic output (e.g., `--58554--`) or make it deterministic