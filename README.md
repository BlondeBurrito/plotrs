![linux](https://github.com/BlondeBurrito/plotit/actions/workflows/build_linux.yml/badge.svg)
![windows](https://github.com/BlondeBurrito/plotit/actions/workflows/build_windows.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/plotit.svg)](https://crates.io/crates/plotit)
[![docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/plotit)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/d/plotit.svg)](https://crates.io/crates/plotit)

# plotit

A CLI app for plotting data sets onto a variety of graphs.

## Install

`cargo install plotit`

## How To Use

Controls formatting settings. I have a prefernce for using tabs simply because in shared projects individuals have their own preference for indentation depth and so automatic tab resizing can make a code base gentler on the eyes.

```bash
plotit -g <graph_type> -c <path_to_config_ron_file> -o <dir_for_output_png>
```

E.g

```bash
plotit -g scatter -c examples/scatter/scatter.ron -o examples/scatter
```

Note that if your canvas is too small then your title and axis label test will become blurry.

## Examples

Currently commented out, as I use clippy more I suspect to customise what it does.

<img src="docs/abstract_example.png" alt="abs" width="370"/>

## LICENSE

[Dual license of MIT and Apache](https://github.com/BlondeBurrito/plotit/blob/main/LICENSE).
