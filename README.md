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

- [x] [Application](docs/elm_app.md)
- [x] [Widgets](https://www.enlightenment.org/_legacy_embed/widgetslist.html)
  - [x] [Containers](docs/elm_containers.md)
    - [x] [Window](docs/elm_containers.md#Window) - Primari application window
    - [x] [NaviFrame](docs/elm_containers.md#NaviFrame)
    - [x] [Box](docs/elm_containers.md#Box) - Basic horizontal/vertical packing
    - [x] [Bubble](docs/elm_containers.md#Bubble) - Basic container with title
    - [x] [Frame](docs/elm_containers.md#Frame) - Framed container with title
    - [x] [Popup](docs/elm_containers.md#Popup) - Overlays and dialogs
  - [x] [Outputs](docs/elm_outputs.md)
    - [x] [Label](docs/elm_outputs.md#Label) - Text and icon display
    - [x] [Separator](docs/elm_outputs.md#Separator)
    - [x] [ProgressBar](docs/elm_outputs.md#ProgressBar)
  - [x] [Inputs: String](docs/elm_inputs.md)
    - [x] [Entry](docs/elm_inputs.md#Entry) - Basic text input
  - [x] [Triggers](docs/elm_triggers.md)
    - [x] [Button](docs/elm_triggers.md#Button) - Standart push button
  - [x] [Rangers: f64](docs/elm_outputs.md)
    - [x] [Spinner](docs/elm_ranges.md#Spinner) - Numeric input with arrows
    - [x] [Slider](docs/elm_ranges.md#Slider) - Value slider
  - [x] [Selectors: Vec<String>](docs/elm_selectors.md)
    - [x] [Menu](docs/elm_selectors.md#Menu)
    - [x] [List](docs/elm_selectors.md#List) - Simple vertical item list
