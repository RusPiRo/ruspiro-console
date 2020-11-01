# Changelog

## :peach: v0.4.0

- ### :wrench: Maintenance

  - using a better pipeline setup for build and release.
  - using the `core::fmt::Write` as trait bound for anything that might act as a console output instead of a custom trait.

## :apple: v0.3.1

- ### :bulb: Features

- ### :detective: Fixes

  - writing to the console is no longer a blocking operation. This allows console outputs from within Interrupt handlers without possible deadlocks. However, concurrent console writes may lead to "garbage" on the console output (terminal)
