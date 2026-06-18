# efl-rs

Rust bindings for the [Enlightenment Foundation Libraries](https://www.enlightenment.org/about-efl).

## Other bindings for EFL

- [Python](https://github.com/DaveMDS/python-efl)
- [Vala](https://github.com/freesmartphone/libeflvala)

## Alternatives

- [FLTK-rs](https://github.com/fltk-rs)
- [GTK-rs](https://github.com/gtk-rs)
- [RSTK](https://codeberg.org/peterlane/rstk)
- [FoxTK-rs](https://github.com/theavege/foxtk-rs)

## [Dependencies](https://www.enlightenment.org/docs/distros/start)

- [Linux](.github/workflows/make.sh)
- [Windows](.github/workflows/make.ps1)

## Work in process

- [x] [Widgets](https://www.enlightenment.org/_legacy_embed/widgetslist.html)
  - [x] [Containers](docs/elm_containers.md)
    - [x] [Box](docs/elm_containers.md#Box) - Basic horizontal/vertical packing
    - [x] [NaviFrame](docs/elm_containers.md#NaviFrame) - Slided container
    - [x] [Bubble](docs/elm_containers.md#Bubble) - Container with title
    - [x] [Frame](docs/elm_containers.md#Frame) - Collapsed container
    - [x] [Popup](docs/elm_containers.md#Popup) - Modal container
  - [x] [Outputs](docs/elm_outputs.md)
    - [x] [Separator](docs/elm_outputs.md#Separator) - Display horizontal/vertical line
    - [x] [ProgressBar](docs/elm_outputs.md#ProgressBar) - Display progress
  - [x] [Inputs](docs/elm_inputs.md)
    - [x] [Entry (String)](docs/elm_inputs.md#Entry) - Change text
    - [x] [Check (bool)](docs/elm_triggers.md#Check) - Change option
    - [x] [Rangers ((f64..=f64), f64)](docs/elm_outputs.md) - Change float
      - [x] [Spinner](docs/elm_ranges.md#Spinner)
      - [x] [Slider](docs/elm_ranges.md#Slider)
  - [x] [Selectors (Vec<String>, u32)](docs/elm_selectors.md)  - Select variant
    - [x] [Radio](docs/elm_selectors.md#Radio) - Classic selector
    - [x] [List](docs/elm_selectors.md#List) - Vertical scrollable selector
    - [x] [SegmentControl](docs/elm_selectors.md#SegmentControl) - Horizontal selector
    - [x] [Menu](docs/elm_selectors.md#Menu) - Popup selector
  - [x] [Triggers](docs/elm_triggers.md)
    - [x] [Button](docs/elm_triggers.md#Button)

# Scrot

![enlightenment](https://www.enlightenment.org/_media/aa/shot-2021-12-13_17-49-55.png)
