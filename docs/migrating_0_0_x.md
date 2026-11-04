# Migrating from 0.0.x to 0.1.0

This guide records the intended migration for the first stable release. Review
it against the final 0.1.0 changelog before upgrading: the release has not yet
been cut.

## Check fallible operations

Use `try_*` methods when values come from users, configuration, or other
untrusted input. In particular, C strings cannot contain NUL bytes, and fallible
methods return `EflResult<T>` instead of panicking during conversion. Match
`EflError` or propagate it with `?`.

## Keep objects on the EFL thread

Widget, timer, and item wrappers are `!Send` and `!Sync`. Code that previously
moved widget references between threads should send data instead and use
`spawn_main` to schedule UI updates on the event-loop thread.

## Retain callback owners

Keep `Timer` and `ComponentMount` handles alive for as long as their callbacks
should run. Dropping or stopping those handles releases the callback state.
Widget callbacks are released when the native widget is deleted.

## Raw-pointer construction

Raw-pointer constructors are `unsafe`. Existing integrations using `from_raw`
need an explicit safety argument that the pointer is valid, has the expected
EFL type, and is used only on the EFL main thread.

## Verify selector behavior

Use `length()` for item counts and the corrected `clear()` implementations for
`List`, `Menu`, and `SegmentControl`. Check `is_set()` before using default
(empty) wrappers; use fallible methods when available.
