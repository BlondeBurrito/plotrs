# plotit

A CLI for plotting data sets onto a variet of graphs.

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

## Examples

Currently commented out, as I use clippy more I suspect to customise what it does.

<img src="docs/abstract_example.png" alt="abs" width="370"/>

## LICENSE

[Dual license of MIT and Apache](https://github.com/BlondeBurrito/plotit/LICENSE).
