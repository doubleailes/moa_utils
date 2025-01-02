# MOA Calculation Library

This library provides utilities for calculating the Minute of Angle (MOA) for a given drop (in m) and distance (in meters), as well as calculating the drop (in cm) for a given MOA and distance (in meters).

The system assumes everything is in meters.

## Usage

`cargo add angle_calculation`

## Examples

### Calculate angle from MOA

```rust
use angle_calculation::MOADD;

let drop = 0.1;
let distance = 100.0;

let MOA_struct = MOADD::new_from_drop_distance(drop, distance);
let MOA = MOA_struct.get_moa();
println!("MOA: {}", MOA);
```

### Calculate angle from MRAD

```rust
use angle_calculation::MRADDD;

let drop = 0.1;
let distance = 100.0;

let MRAD_struct = MRADDD::new_from_drop_distance(drop, distance);
let MRAD = MRAD_struct.get_mrad();
println!("MRAD: {}", MRAD);
```
