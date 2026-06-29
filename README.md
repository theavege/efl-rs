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

## Screenshots

![enlightenment](https://www.enlightenment.org/_media/aa/shot-2021-12-13_17-49-55.png)

```plantuml
@startuml
!theme sunlust
skinparam linetype ortho
right footer UML: Class Diagram
left to right direction
package "efl" #line.dashed {
    package containers #line.dashed {
        struct Box {
            -ptr : Pointer
        }
        struct Buble {
            -ptr : Pointer
        }
        struct Frame {
            -ptr : Pointer
        }
        struct Naviframe {
            -ptr : Pointer
        }
        struct Panes {
            -ptr : Pointer
        }
        struct Window {
            -ptr : Pointer
        }
    }
    package outputs #line.dashed {
        struct Label {
            -ptr : Pointer
        }
        struct ProgressBar {
            -ptr : Pointer
        }
        struct Separator {
            -ptr : Pointer
        }
    }
    package inputs #line.dashed {
        struct Button {
            -ptr : Pointer
        }
        struct Check {
            -ptr : Pointer
        }
        struct Entry {
            -ptr : Pointer
        }
        struct List {
            -ptr : Pointer
        }
        struct Radio {
            -ptr : Pointer
        }
        struct Slider {
            -ptr : Pointer
        }
        struct Spinner {
            -ptr : Pointer
        }
        struct SegmentControl {
            -ptr : Pointer
        }
    }
    package prelude #line.dashed {
        interface WidgetExt {
            #ptr as_raw()
            #from_raw(ptr)
            #del()
            #show()
        }
        interface TextExt {
            +str text()
            +set_text(str)
        }
        interface ContainerExt {
            #add_item(WidgetExt)
        }
        interface OrientExt {
            +Self with_horizontal()
        }
        interface InputsExt {
            +call_signal(signal)
            +do_callback()
            +bool focus()
            +type value()
            +set_value(type)
            +Self  with_value(type)
            +set_disabled(bool)
            +Self with_signal(signal)
            +Self with_callback()
            +Self with_cursor(cursor)
            +Self with_tooltip(tooltip)
        }
        interface RangerExt {
            +Self with_range(f64, f64)
            +Self with_step(f64)
            +Self with_format(str)
        }
        interface SelectorExt {
            +u32 lenght()
            +Self with_items([str])
        }
        interface OrientationExt extends WidgetExt
        interface ContainerExt extends WidgetExt
        interface TextExt extends WidgetExt
        interface InputsExt extends WidgetExt
        interface RangerExt extends InputsExt
        interface SelectorExt extends InputsExt
    }
    struct outputs.Label implements prelude.TextExt

    struct outputs.Separator implements prelude.OrientationExt

    struct containers.Box implements prelude.OrientationExt
    struct containers.Box implements prelude.ContainerExt

    struct containers.Buble implements prelude.ContainerExt

    struct containers.Frame implements prelude.TextExt
    struct containers.Frame implements prelude.OrientationExt

    struct containers.Panes implements prelude.ContainerExt
    struct containers.Panes implements prelude.OrientationExt

    struct containers.Window implements prelude.ContainerExt

    struct inputs.Button implements prelude.TextExt
    struct inputs.Check implements prelude.TextExt
    struct inputs.Radio implements prelude.TextExt
    struct inputs.Button implements prelude.InputsExt

    struct inputs.Entry implements prelude.InputsExt

    struct inputs.Slider implements prelude.RangerExt
    struct inputs.Slider implements prelude.OrientationExt

    struct inputs.Spinner implements prelude.RangerExt

    struct inputs.List implements prelude.SelectorExt
    struct inputs.SegmentControl implements prelude.SelectorExt
}
@enduml
```
