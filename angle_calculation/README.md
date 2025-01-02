# MOA Calculation Library

This library provides utilities for calculating the Minute of Angle (MOA) for a given drop (in cm) and distance (in meters), as well as calculating the drop (in cm) for a given MOA and distance (in meters).

## Usage

`cargo add angle_calculation`

## Examples

```rust
use angle_calculation::angle_calculation;

let drop = 10.0;
let distance = 100.0;

let MOA_struct = MOADD::new_from_drop_distance(drop, distance);
let MOA = MOA_struct.get_moa();
println!("MOA: {}", MOA);
```
