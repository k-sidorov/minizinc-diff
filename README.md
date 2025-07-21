# minizinc-diff

A CLI tool for differential testing of MiniZinc solvers. Use it to:
- Compare solver outputs (e.g., default vs. alternative search strategies or solver A vs. solver B).
- Generate benchmarks for fuzz testing.

## ✨ Features

- **Automated instance generation** for selected model templates.
- **Parallel solver comparison** with timeouts.
- **Pretty (colorful) or plain output**, auto-detected like `grep`.
- **Diff-style reports** of solution mismatches.
- **Exit codes** for automation and scripting support.

## ⚙ Installation

```bash
cargo install --git https://github.com/k-sidorov/minizinc-diff
```

Make sure `minizinc` is available in your `$PATH`.


## 📄 Usage

### Generate Instances

```bash
minizinc-diff generate ./minizinc-fuzz-testing/ market-split:100:2:30 market-split:100:1:20
```

This creates multiple model/data pairs in the given directory. You can then fuzz different solver configurations on these.

### Compare Solver Outputs

```bash
minizinc-diff diff \
  ./minizinc-fuzz-testing/market_split/model.mzn \
  ./minizinc-fuzz-testing/market_split/2_25_100_1.dzn \
  gecode \
  gecode:--free-search
```

This compares solutions found by Gecode's default and `--free-search` configurations.

You can also run it in a Slurm job or redirect output as needed:

```bash
minizinc-diff test model.mzn data.dzn or-tools gecode > results.log
```

## ❓ Exit Codes

| Code | Meaning                  |
| ---- | ------------------------ |
| 0    | Success, solutions match |
| 255  | Mismatch found           |
| 1    | Left solver crash  |
| 2    | Right solver crash |
| 3    | Both solvers crash |
| 5    | Left solver timeout  |
| 6    | Right solver timeout |
| 7    | Both solvers timeout |


## ✉ License

This project is licensed under the terms of the **GNU General Public License v3.0**. See `LICENSE` for details.


## 💼 Contributions

Bug reports and PRs welcome! Feel free to open an issue if you'd like support for new models, additional output formats (e.g., JSON), or better instance generators.

---

Made with ❤ by [@k-sidorov](https://github.com/k-sidorov)
