# Base Converter (Rust/EXE)

## Usage
1. Enter an integer you want to convert (e.g., `1`, `25`, `1FF`, `aF2`).
2. Enter the number's base (can be default or custom).
3. Enter the base you want to convert the number to (the base can be default or custom).

## Rules
1. Number you want to convert must be positive integer.
2. Default base should be entered as its length/amount of digits (e.g., decimal has `10` elements so `10` should be entered)
3. Custom base must be enetered as its every digit in order and without any spacing.
4. If your custom base contins (decimal) digits only, use `__` (2 underscored) to force custom base.
5. Every charcter of custom base is a distinct digit.

## Warning
EXE file has no signature, so it might trigger Windows smartscreen.
