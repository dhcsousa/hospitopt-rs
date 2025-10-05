![CI](https://github.com/dhcsousa/hospitopt-rs/actions/workflows/ci.yml/badge.svg)

# hospitopt-rs

Rust-based optimization project that uses constraint programming to maximize the number of lives saved in emergency and healthcare scenarios. It models hospitals, diseases, available beds, ambulance positions and capacities, and patient needs, then computes optimized resource allocations to improve medical response and outcomes.

## Vision

- Capture hospitals, treatments, bed capacity, ambulance fleets, and patient demand in a unified Rust model.
- Explore Google OR-Tools CP-SAT (via the `cp_sat` bindings) to optimize triage, routing, and resource allocation.
