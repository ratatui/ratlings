# Contributing

Contributions are welcome. Each exercise should teach one focused Ratatui idea,
include tests that describe completion, provide a useful hint and have a
working solution.

## Add an exercise

1. Add its metadata to `info.toml` in curriculum order.
2. Add matching files at `exercises/exerciseN.rs` and `solutions/exerciseN.rs`.
3. Keep terminal I/O out of tests. Prefer `ratatui::backend::TestBackend` or
   render directly into a `ratatui::buffer::Buffer`.
4. Regenerate the binary list and validate everything:

   ```sh
   rustlings dev update
   rustlings dev check
   cargo fmt --check
   ```

Each step should carry the working kitchen dashboard forward and add one small
change. Keep Gustau's intro to one short line. Exercises should start incomplete
while every solution compiles and passes. Use a concise hint when needed.
