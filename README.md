![linux](https://github.com/BlondeBurrito/plotit/actions/workflows/build_linux.yml/badge.svg)
![windows](https://github.com/BlondeBurrito/plotit/actions/workflows/build_windows.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/plotit.svg)](https://crates.io/crates/plotit)
[![docs](https://img.shields.io/badge/docs-docs.rs-orange.svg)](https://docs.rs/plotit)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/d/plotit.svg)](https://crates.io/crates/plotit)

# plotit

A CLI app for plotting data sets onto a variety of graphs.

## Features

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

Note that if your canvas is too small then your title and axis labels may become blurry.

## Graph `.ron` Schemas

## Symbol Types

## Best Fit Schemas

## Examples

### [Simple Scatter](https://github.com/BlondeBurrito/plotit/tree/main/examples/scatter)

<img src="examples/scatter/engery_against_time_for_fuzzing_about_things.png" alt="s" width="400"/>

### [Image Size Scales Elements Dynamically](https://github.com/BlondeBurrito/plotit/tree/main/examples/scatter_large)

Based on the dimensions of your image (`canvas_size`) the text and axes positions are automatically calculated. You can also toggle a light grey background grid drawn the from axes scales.

<img src="examples/scatter_large/a_large_graph_for_testing_positions_of_graph_elements_and_stuff.png" alt="s" width="800"/>

### [Scatter Multidata](https://github.com/BlondeBurrito/plotit/tree/main/examples/scatter_multidata_one_csv)

From single or multiple `csv` files you can plot several data sets onto a single graph. Each data set can be configured to plot with a different colour and/or symbol. The legend can be toggled on and off. The size and thickness of the symbols are configurable on a per data set basis.

From a single `csv` containing multiple columns for different data sets:

<img src="examples/scatter_multidata_one_csv/oh_wow__multiple_data_sets.png" alt="s" width="800"/>

From two `csv` files where each contains a column pair:

<img src="examples/scatter_multidata_two_csv/data_from_two_csv_files_woweeeee.png" alt="s" width="800"/>

### [Scatter Error Bars](https://github.com/BlondeBurrito/plotit/tree/main/examples/scatter_error_bars)

You can also indicate uncertainty with the use of error bars which can be specified for either axes.

<img src="examples/scatter_error_bars/we_have_some_certainty_in_y.png" alt="s" width="800"/>
<img src="examples/scatter_error_bars/we_have_some_certainty_in_x_and_y.png" alt="s" width="800"/>

## Contributing

## LICENSE

[Dual license of MIT and Apache](https://github.com/BlondeBurrito/plotit/blob/main/LICENSE).
