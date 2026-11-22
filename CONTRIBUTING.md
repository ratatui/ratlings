# Contributing

Contributions are welcome. Each exercise should teach one focused Ratatui idea,
include tests that describe completion, provide a useful hint and have a
working solution.

## Add an exercise

1. Add its metadata to `info.toml` in curriculum order.
2. Add matching files under `exercises/<section>/` and `solutions/<section>/`.
3. Keep terminal I/O out of tests. Prefer `ratatui::backend::TestBackend` or
   render directly into a `ratatui::buffer::Buffer`.
4. Regenerate the binary list and validate everything:

   ```sh
   rustlings dev update
   rustlings dev check
   cargo fmt --check
   ```

Exercises should start incomplete (usually with `todo!()`), while every
solution must compile and pass. Use links to the official Ratatui documentation
in hints when a concept needs more background.
