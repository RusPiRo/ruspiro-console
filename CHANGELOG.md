# Changelog

## :peach: v0.4.0

- ### :wrench: Maintenance

  - using a better pipeline setup for build and release.
  - using the `core::fmt::Write` as trait bound for anything that might act as a console output instead of a custom trait.
  - as the `Write` trait requires the structure to be mutable when writing to the output channel the console operations are blocking operations -> thus requiring atomic operations to be possible -> thus requiring the *MMU* to be activated before using the console abstraction is possible. Otherwise execution will *hang* as atomics are not able to be processed by the CPU.

## :apple: v0.3.1

- ### :bulb: Features

- ### :detective: Fixes

  - writing to the console is no longer a blocking operation. This allows console outputs from within Interrupt handlers without possible deadlocks. However, concurrent console writes may lead to "garbage" on the console output (terminal)
