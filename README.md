Example code for building and running eBPF programs with `aya`.

`deployable` is the "loader" and other projects are various eBPF programs.

This is not in a Cargo workspace due to each project (might) need different profile when buliding, which is not supported in Cargo workspace.
