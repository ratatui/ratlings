<!-- WRITE ME
  SLOT:   Chapter title.
  GOAL:   Name this outing. Sections are separate trips, so this title should
          read as a place or a departure, not as "Chapter 1".
-->
# Packing

<!-- WRITE ME
  SLOT:   The story beat that opens this trip.
  GOAL:   Establish the rat, where it is going, and the constraint that drives
          the whole section: the pack is a fixed size and the gear is not
          negotiable. The reader should finish this and understand that
          packing is an allocation problem before they ever see the word
          `Constraint`.
  LENGTH: ~1-2 short paragraphs.
-->

## What you'll learn

- The difference between space you **reserve** and space you let something
  **claim** — and why a layout needs both.
- That constraints are read in the order things appear on screen, top to
  bottom, not the order you'd physically pack them.
- That a layout is not a one-off measurement. It gets recomputed every time
  the terminal is resized, so "whatever is left over" has to be expressed as
  a rule rather than a number.

## Exercises

| Exercise | What it covers |
| --- | --- |
| `lesson1` | Drawing your first widget: `render_widget`, and a `Block` with borders and a title. |
| `lesson2` | Splitting a fixed area into compartments with `Layout` and `Constraint`. |
