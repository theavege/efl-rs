# efl-rs

Work in progress Rust bindings for the [Enlightenment Foundation Libraries](https://www.enlightenment.org/about-efl).

-[refl-sys]-> refl.prelude -> refl.lib -> example

## [Dependencies](.github/workflows/make.sh)

## Other bindings for [EFL](https://www.enlightenment.org/about-efl)

- [Python](https://docs.enlightenment.org/python-efl/current)
- [Vala](https://github.com/freesmartphone/libeflvala)

## Alternatives

- [FLTK-rs](https://github.com/fltk-rs)
- [GTK-rs](https://github.com/gtk-rs)
- [RSTK](https://codeberg.org/peterlane/rstk)
- [FoxTK-rs](https://github.com/theavege/foxtk-rs)

## Work in process

- [ ] Ecore
  - [ ] [Con](https://docs.enlightenment.org/python-efl/current/ecore/module-ecore_con.html)
  - [x] Event
  - [x] [Timer](https://docs.enlightenment.org/python-efl/current/ecore/class-timer.html)
- [ ] [Emotion](https://docs.enlightenment.org/python-efl/current/emotion/emotion.html)
- [ ] Elementary
  - [x] [Application](https://www.enlightenment.org/develop/legacy/api/c/start#infralist.html)
  - [ ] [Containers](https://www.enlightenment.org/develop/legacy/api/c/start#containerslist.html)
    - [x] [Box](https://docs.enlightenment.org/python-efl/current/elementary/box.html)
    - [x] [Bubble](https://docs.enlightenment.org/python-efl/current/elementary/bubble.html)
    - [x] [Frame](https://docs.enlightenment.org/python-efl/current/elementary/frame.html)
    - [ ] [Flip](https://docs.enlightenment.org/python-efl/current/elementary/flip.html)
    - [ ] [Hover](https://docs.enlightenment.org/python-efl/current/elementary/hover.html)
    - [x] [NaviFrame](https://docs.enlightenment.org/python-efl/current/elementary/naviframe.html)
    - [x] [Notify](https://docs.enlightenment.org/python-efl/current/elementary/notify.html)
    - [ ] [Layout](https://docs.enlightenment.org/python-efl/current/elementary/layout.html)
    - [x] [Panel](https://docs.enlightenment.org/python-efl/current/elementary/panel.html)
    - [x] [Panes](https://docs.enlightenment.org/python-efl/current/elementary/panes.html)
    - [x] [Popup](https://docs.enlightenment.org/python-efl/current/elementary/popup.html)
    - [x] [Scroller](https://docs.enlightenment.org/python-efl/current/elementary/scroller.html)
    - [x] [Window](https://docs.enlightenment.org/python-efl/current/elementary/window.html)
  - [ ] [Widgets](https://www.enlightenment.org/_legacy_embed/widgetslist.html)
    - [x] Rangers
      - [x] [Spinner](https://docs.enlightenment.org/python-efl/current/elementary/spinner.html)
      - [x] [Slider](https://docs.enlightenment.org/python-efl/current/elementary/slider.html)
    - [x] Selectors
      - [x] [ActionSlider](https://docs.enlightenment.org/python-efl/current/elementary/actionslider.html)
      - [x] [Calendar](https://docs.enlightenment.org/python-efl/current/elementary/calendar.html)
      - [ ] [FileSelector](https://docs.enlightenment.org/python-efl/current/elementary/fileselector.html)
      - [ ] [DaySelector](https://docs.enlightenment.org/python-efl/current/elementary/dayselector.html)
      - [ ] [DiskSelector](https://docs.enlightenment.org/python-efl/current/elementary/diskselector.html)
      - [ ] [ColorSelector](https://docs.enlightenment.org/python-efl/current/elementary/colorselector.html)
      - [x] [Ctxpopup](https://docs.enlightenment.org/python-efl/current/elementary/ctxpopup.html)
      - [ ] [Combobox](https://docs.enlightenment.org/python-efl/current/elementary/combobox.html)
      - [x] [FlipSelector](https://docs.enlightenment.org/python-efl/current/elementary/flipselector.html)
      - [x] [HoverSel](https://docs.enlightenment.org/python-efl/current/elementary/hoversel.html)
      - [x] [Menu](https://docs.enlightenment.org/python-efl/current/elementary/menu.html)
      - [x] [List](https://docs.enlightenment.org/python-efl/current/elementary/list.html)
      - [x] [Radio](https://docs.enlightenment.org/python-efl/current/elementary/radio.html)
      - [x] [SegmentControl](https://docs.enlightenment.org/python-efl/current/elementary/segment_control.html)
      - [x] [Toolbar](https://docs.enlightenment.org/python-efl/current/elementary/toolbar.html)
    - [x] Triggers
      - [x] [Button](https://docs.enlightenment.org/python-efl/current/elementary/button.html)
      - [x] [Clock](https://docs.enlightenment.org/python-efl/current/elementary/clock.html)
    - [x] Input
      - [x] [Entry](https://docs.enlightenment.org/python-efl/current/elementary/entry.html)
    - [x] Output
      - [ ] [Image](https://docs.enlightenment.org/python-efl/current/elementary/image.html)
      - [x] [Label](https://docs.enlightenment.org/python-efl/current/elementary/label.html)
      - [x] [Separator](https://docs.enlightenment.org/python-efl/current/elementary/separator.html)
      - [x] [ProgressBar](https://docs.enlightenment.org/python-efl/current/elementary/progressbar.html)
    - [x] Misc
      - [x] [Icon](https://docs.enlightenment.org/python-efl/current/elementary/icon.html)
      - [x] [Cursor](https://docs.enlightenment.org/python-efl/current/elementary/cursor.html)
      - [x] [Tooltip](https://docs.enlightenment.org/python-efl/current/elementary/tooltip.html)

- [ ] Tutorials
  - [ ] [7GUIs](https://7guis.bradwoods.io)
  - [ ] [MediaPlayer](https://www.enlightenment.org/develop/legacy/tutorial/multimedia_tutorial)
  - [ ] PictureViewer
