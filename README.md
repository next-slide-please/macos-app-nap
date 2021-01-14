# Preventing macOS AppNap

On macOS, [AppNap][apple-doc] is a mechanism of reducing the energy consumption of apps by suspending them if the user is not interacting with an app.

As a concrete example, when running the [druid timer example][druid-timer] with some debug output added around [the timer event][druid-time-line], you'll notice that the output slows down significantly after about a minute if the application windows is in the background an completely invisible / covered by other windows.

To prevent this, e.g. if you're doing processing in the background and are relying on the timer, you can use this crate. Currently it only supports disabling App Nap entirely for the applications. More granular control is likely possible and desirable, and may be added in the future (pull requests welcome!).

## Usage

Fairly simple at the moment:

```rust
fn main() {
    macos_app_nap::prevent();
}
```

## Potential improvements

* The C helper could be replaced by using/adapting the [audio_thread_priority][audio-thread-priority] crate, which appears to be doing something very similar.

[apple-docs]: https://developer.apple.com/library/archive/documentation/Performance/Conceptual/power_efficiency_guidelines_osx/AppNap.html
[druid-timer]: https://github.com/linebender/druid/blob/master/druid/examples/timer.rs
[druid-timer-line]: https://github.com/linebender/druid/blob/51ccd6542e15c6c11b10c0b8d609789045fee337/druid/examples/timer.rs#L56
[audio-thread-priority]: https://github.com/padenot/audio_thread_priority


## License

MIT. See LICENSE.md for details.