Changelog
=========

## Version 2.0.1 (In Progress)
* Migrates Repository to GitHub

## Version 2.0.0 (2021-09-27)
* Refactored API, divided into "add" for mutable and "with" for chainable
* Fixed non-generic fields which snuck by in 1.1.0
* Refactored types to use strings instead of boxed intermediate types
    * While we're still using the heap, we cut out one level of indirection which increases efficiency
    * This is entirely transparent to end users

## Version 1.1.0 (2021-06-17)
* Rise of the Generics! -- `add_x` methods are now much more flexible!
    * Pass any type implementing `ToString` as a required parameter
    * Pass attributes using more than just `HashMap`s (Including arrays with Rust 1.53)
* Further defined escape hatches (for extensibility)
* Added `add_raw` method for containers

## Version 1.0.0 (2021-01-24)
* Initial Release