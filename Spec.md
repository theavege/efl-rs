# efl-rs Specification — Roadmap to v0.1.0 (first stable release)

Status of the crates today: `efltk` / `efltk-sys` **0.0.8** (pre-1.0, 0.x series).
The widget coverage goal is already met — every widget listed in the README
[Transition Table](README.md#transition-table) has a safe wrapper (`Window`,
`Box`, `Table`, `Scroller`, `Panes`, `Naviframe`, `Frame`, `Button`, `Check`,
`Radio`, `Entry`, `Label`, `Slider`, `Spinner`, `List`, `Menu`, `Genlist`,
`Toolbar`, `Hoversel`, `Diskselector`, `SegmentControl`, `Calendar`, `Popup`,
`Notify`, `Bg`, `Panel`, `Photo`, `Datetime`, `Grid`, `Flip`, `Hover`,
`Ctxpopup`, `Index`, `Dayselector`, `Actionslider`, `Bubble`, `Photocam`,
`Slideshow`, `Gengrid`, `Map`, `Video`, `Web`, `Glview`, `Conformant`,
`Layout`, `Multibuttonentry`, `Combobox`, …).

What is missing for *stable* is not more widgets, but **API guarantees**:
error handling without panics, documented lifetimes/threading, a frozen public
API surface, and a release process. This document is the plan.

## Definition of "stable" (exit criteria for v0.1.0)

1. No reachable `panic!` from normal API misuse paths — fallible APIs return
   `EflResult<T>` instead.
2. Public API freeze: `cargo doc` shows no undocumented `pub` items; breaking
   changes after 0.1.0 require a semver-compatible justification or a new
   minor/major version.
3. CI green on Linux, macOS, Windows (MSYS2) for build, tests, examples,
   `clippy -D warnings`, and `cargo doc`.
4. Documented threading model and object-lifetime ownership rules with tests
   that enforce them.
5. CHANGELOG finalized, version bumped 0.0.8 → 0.1.0 in both crates, published
   to crates.io with a tagged git release `v0.1.0`.

## Phase 1 — Error handling & safety hardening

- [ ] **Audit panic sites.** Inventory every `expect_cstring`, `unwrap`,
      `expect`, and raw-pointer dereference in `efltk/src/lib.rs` and
      `efltk/src/prelude.rs`; classify as " programmer error (keep panic)" vs
      "runtime condition (must return `Err`)".
- [ ] **Extend the `try_*` pattern.** The prelude already has
      `try_set_text`, `try_with_text`, `try_set_part`, `try_with_part`,
      `WindowExt::try_new`. Add `try_*` equivalents (or convert the base
      methods) for all string-accepting setters: tooltips, labels/formats,
      item text (`SelectorExt::add`, `add_items`), file paths
      (`FileSelector`, `FileEntry`, `Image`), Edje part names (`set_part`).
- [ ] **Null-pointer guards.** Every method must no-op or return `Err` on an
      empty wrapper (`is_set() == false`) — extend the `Naviframe::promote`
      fix pattern crate-wide; add a debug assertion helper.
- [ ] **Callback safety.** Verify all C callbacks (`smart_cb`, `ecore_task_cb`,
      gen-item classes, drop cbs) catch panics at the FFI boundary and free
      boxed closures exactly once (no double-free on `del()` + timer stop).
- [ ] **`unsafe` review.** All raw-pointer constructors are already `unsafe`;
      add `SAFETY:` comments and check each `unsafe fn` documents its pointer
      validity requirements (rustdoc lint `missing_docs` + clippy).

## Phase 2 — Lifecycle & ownership model

- [ ] **Decide parent/child ownership semantics** and document it: who deletes
      what when (`Window` owns the tree; `WidgetItem` borrows its parent
      widget; `Timer`/`ComponentMount` own their callbacks). Current model is
      handle-based (`Option<NonNull<_>>`); keep it, but write it down.
- [ ] **Parent lifetime:** evaluate making widget constructors take `&impl
      WidgetExt` parents with lifetime-tied wrappers, or explicitly document
      that wrappers do not borrow the parent and deleting a parent invalidates
      children (with a `debug_assert` via `elm_object_parent_of` checks).
- [ ] **Threading contract.** Wrappers are `!Send`/`!Sync` (done); document
      "EFL objects live on the main loop thread", and define the sanctioned way
      to marshal work back to the main thread (`Timer` / `ecore` main-loop
      tick). Consider a small `spawn_main(fn)` helper backed by
      `ecore_main_loop_thread_safe_call_async`.
- [ ] **`run` / `exit` semantics.** Document re-entrancy, exit-from-callback,
      and what happens if `run` is called twice; make `run` return
      `EflResult<()>` so init failure doesn't abort.

## Phase 3 — API polish & consistency

- [ ] **Naming sweep.** Uniform `with_*` (builder), `set_*`/getter pairs,
      `add_*`/`clear`/`length` for item containers; deprecate leftovers (e.g.
      typos like any remaining `lenght`, inconsistent `Glview` vs `GLView`
      casing, `OrientExt::with_horizontal(bool)` reading like a setter).
- [ ] **Trait hierarchy cleanup.** Freeze which widgets implement which traits
      (`WidgetExt`, `TextExt`, `ContainerExt`, `InputExt<T>`, `RangerExt`,
      `SelectorExt`, `OrientExt`, `Update<T>`); remove accidental blanket impls
      before the freeze. Split the monolithic `lib.rs`/`prelude.rs` into
      modules (`widgets/`, `traits/`) — cosmetic, but improves docs navigation.
- [ ] **Feature-gate heavy extras.** Put `Web`, `Map`, `Video`, `Photocam`
      behind optional cargo features (`web`, `map`, `video`, `photocam`) so the
      default build only needs elementary/evas/ecore/eina, matching typical
      distro packaging.
- [ ] **`efltk-sys` hygiene.** Pin supported EFL version range (e.g. EFL ≥
      1.25, ≤ current stable), have `build.rs` detect and report the found
      version, and blocklist any additional generated symbols that leak libc
      internals (follow-up to the 0.0.8 `malloc/memcmp/...` blocklist fix).

## Phase 4 — Documentation

- [ ] **Doc comments on 100% of public items**, each with a `# Examples` block
      runnable via `cargo test --doc` (use `#[cfg]`/`no_run` where a display
      server is unavailable).
- [ ] **Book-style guides in `docs/`:** getting started, threading model,
      memory/lifetime model, migrating from GTK (generated from the README
      Transition Table), migrating from 0.0.x → 0.1.0.
- [ ] **README:** keep "First window" example compiling against the release;
      add MSRV policy (currently Rust 1.85 / edition 2024) and support matrix
      (OS × EFL version).
- [ ] **CHANGELOG:** deduplicate the existing entries (0.0.6/0.0.7 blocks are
      repeated and the Unreleased section mixes two "Added"/"Changed" groups),
      then cut `[0.1.0]`.

## Phase 5 — Testing & CI

- [ ] **Headless UI smoke tests.** Run the four examples (`first_window`,
      `simple`, `snake`, `floppy_bird`) under Xvfb/Wayland-headless in CI with
      a timeout; assert clean startup/shutdown (catches regressions the unit
      tests can't).
- [ ] **Leak/callback tests.** Instrument allocation of boxed callbacks and
      `Eina_List`s; assert deletion frees them (extend existing `tests.rs`).
- [ ] **`#![deny(missing_docs, missing_debug_implementations)]`** and
      `clippy::pedantic` triage with an explicit allow-list committed as
      policy.
- [ ] **Public-API diff check.** Add `cargo public-api` (or `cargo-machete`
      for deps) to CI so any API change after the freeze fails the build until
      reviewed.
- [ ] **Platform matrix:** keep Linux/macOS/Windows builds green including the
      MSYS2 EFL path used by CI today.

## Phase 6 — Release process

- [ ] Version bump chore: `efltk` and `efltk-sys` `0.0.8 → 0.1.0`, dependency
      pin `version = "=0.1.0"` updated, README install snippet updated.
- [ ] `cargo publish --dry-run` both crates (verify `exclude` lists don't strip
      needed files; note `readme = "../README.md"` packaging).
- [ ] Publish `efltk-sys` first, then `efltk`; tag `v0.1.0`; GitHub release
      notes generated from CHANGELOG.
- [ ] Post-release: open tracking issue for 0.2 candidates (deferred items
      below).

## Deferred beyond 0.1.0 (explicitly out of scope)

- Async runtime integration (`efl-core` event streams).
- Auto-generated bindings coverage for non-Elementary EFL classes (`Evas`,
  `Edje`, `Ecore` surfaces beyond current usage).
- Accessibility (Atspi) and input-method exposure.
- Theming/editor tooling (`Elm.Layout` EDJ editing helpers).
- 1.0 stabilization cycle (API audit round 2, real-world app feedback).

## Suggested ordering & effort

| Order | Milestone | Rough effort |
| ----- | --------- | ------------ |
| 1 | Phase 1 (errors/safety audit) | largest — touches every widget |
| 2 | Phase 2 (lifecycle/threading) | design-heavy, medium code |
| 3 | Phase 3 (naming, features) | mechanical, medium |
| 4 | Phase 5 (CI smoke tests) | small, enables everything else |
| 5 | Phase 4 (docs) | continuous, finalize last |
| 6 | Phase 6 (release) | small |

> Practical tip: land Phase 5's headless-example CI **first** (it is cheap and
> protects every subsequent refactor), then use it as the safety net while
> converting panics to `EflResult` in Phase 1.
