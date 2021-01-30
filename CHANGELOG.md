# Changelog

## :cat: v0.5.3

Migrate the actual travis-ci build pipeline to github actions

- ### :detective: Fixes

  - fix the release description generation.

## :cat: v0.5.2

Migrate the actual travis-ci build pipeline to github actions

- ### :detective: Fixes

  - fix the version replacements in the github action.

## :cat: v0.5.1

Migrate the actual travis-ci build pipeline to github actions

- ### :wrench: Maintenance

  Updating the minor version to validate the pipeline config end-2-end

## :cat: v0.5.0

Enable the usage of the `log` crate to be used as generic logger facade in other crates to use the static logger/console in this crate.

- ### :bulb: Features
  
  - use the `log` crate to provide a generic logger facade.

## :peach: v0.4.1

- ### :detective: Fixes

  - require the actual structure to be used as `Console` to implement `Send` and `Sync` as this is a requirement of the `Singleton` now.
  
## :peach: v0.4.0

- ### :wrench: Maintenance

  - using a better pipeline setup for build and release.
  - using the `core::fmt::Write` as trait bound for anything that might act as a console output instead of a custom trait.
  - as the `Write` trait requires the structure to be mutable when writing to the output channel the console operations are blocking operations -> thus requiring atomic operations to be possible -> thus requiring the *MMU* to be activated before using the console abstraction is possible. Otherwise execution will *hang* as atomics are not able to be processed by the CPU.

## :apple: v0.3.1

- ### :bulb: Features

- ### :detective: Fixes

  - writing to the console is no longer a blocking operation. This allows console outputs from within Interrupt handlers without possible deadlocks. However, concurrent console writes may lead to "garbage" on the console output (terminal)
