Changelog
=========

## Master
* Fixed non-generic fields which snuck by in 1.1.0

## Version 1.1.0 (2021-06-17)
* Rise of the Generics! -- `add_x` methods are now much more flexible!
    * Pass any type implementing `ToString` as a required parameter
    * Pass attributes using more than just `HashMap`s (Including arrays with Rust 1.53)
* Further defined escape hatches (for extensibility)
* Added `add_raw` method for containers

## Version 1.0.0 (2021-01-24)
* Initial Release