# MOA Calculation Library

This library provides utilities for calculating the Minute of Angle (MOA) for a given drop (in m) and distance (in meters), as well as calculating the drop (in cm) for a given MOA and distance (in meters).

The system assumes everything is in meters.

## Usage

`cargo add angle_calculation`

## Examples

### Calculate angle

```rust
use angle_calculation::AngleDropDistance;

let drop = 0.1;
let distance = 100.0;

let add: AngleDropDistance = AngleDropDistance::from_drop_distance(drop, distance);
let moa = add.get_angle().get_moa();
println!("MOA: {}", MOA);
let mrad = add.get_angle().get_mrad();
println!("MRAD: {}", mrad);
```
