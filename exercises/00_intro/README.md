# Welcome to Ratlings

Before the trip, a look around the kitchen

Ratlings teaches Ratatui one small exercise at a time.
Each lesson is a plain Rust file you edit in your own editor.
A viewer shows your app at the real size of your terminal.
The lesson floats on top.

## What you will learn

- The canvas is, and that ratlings never touches it
- The handful of keys that drive every lesson: esc, C-g, h, ?, n
- The loop the whole course runs on: read, edit, check, move on

## Exercises

| Exercise | What it covers |
|----------|----------------|
| lesson0  | A tour of the viewer and your first edit-and-check

## History

This exercise was originally contributed as PR #3 to the ratatui/ratlings
repository.

It was designed to give students an experience of learning ratatui inside
ratatui itself.

The focus is on how to render lessons in-terminal, giving feedback as you edit,
and figuring out how much to hide versus show to students learning the framework

The contributor wanted to give the student an experience of backcountry
adventures, doing some cooking outside of the kitchen, in the great outdoors
The lessons try to balance abstraction versus showing how the sausage
gets made

## Getting Started

1. Open the viewer with `cargo run --bin lesson0`
2. Work through the checklist above
3. Press i to bring this pane back whenever you close it
4. Everything after this is just more Ratatui

Your first lesson awaits take it one step at a time
