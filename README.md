# Beam Time

Beam time is a logic puzzle where you redirect and split laser beams to create digital circuits.
It's still very unfinished, but I plan to publish it on steam when completed.
Its made with a custom GPU accelerated game engine, just because why not.

Here is an example solution for the XOR level.
This screen recording is a bit out of date, but I don't feel like retaking it.

![beam-time](https://files.connorcode.com/Video/beam_time_gto8PLdFqu.gif)

Below are all the components in the game. From left to right: the emitter, galvo (rotates the mirror it’s facing when powered with a beam), splitter, mirror, delay, wall, and detector.

![Tiles](https://github.com/user-attachments/assets/be140531-560b-491c-be93-ba8bf985dddd)

The game also features a sandbox mode where you can build whatever you want.
Below is an implementation of the one-dimensional elementary cellular automaton [Rule 30](https://en.wikipedia.org/wiki/Rule_30).
In the future I want to build a programmable computer within the sandbox.

![beam_time_USDkpuxkMO](https://github.com/user-attachments/assets/cdd9012a-4f53-487b-9c96-7de6b85262ec)

## Controls

Use WASD or middle mouse + drag to pan and scroll to zoom.

| Key                           | Action                                      |
| ----------------------------- | ------------------------------------------- |
| <kbd>Q</kbd>                  | Copy hovered tile into cursor               |
| <kbd>E</kbd>                  | Toggle state of the hovered emitter         |
| <kbd>R</kbd>                  | Rotates the tile to selection in the cursor |
| <kbd>Shift</kbd>+<kbd>R</kbd> | Same as above but in the opposite direction |
| <kbd>V</kbd>                  | Flip vertically                             |
| <kbd>H</kbd>                  | Flip horizontally                           |
| <kbd>P</kbd>                  | Start (play) simulation                     |
| <kbd>T</kbd>                  | Run cases (when in campaign level)          |
| <kbd>Space</kbd>              | Run one simulation step                     |
| <kbd>+</kbd>                  | Increase simulation speed                   |
| <kbd>-</kbd>                  | Decrease simulation speed                   |
| <kbd>0</kbd>                  | Reset simulation speed                      |
| <kbd>Shift</kbd>+<kbd>0</kbd> | Max simulation speed                        |
