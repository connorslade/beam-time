# Beam Time

Beam time is a logic puzzle about redirecting and splitting laser beams to create circuits.
I'm making a custom GPU accelerated game engine for this, because why not, so this will probably take a while to finish.

Here is an example solution for the XOR level.
Note that this project is still very unfinished.

![beam-time](https://files.connorcode.com/Video/beam_time_gto8PLdFqu.gif)

Below are all the components in the game. From left to right: the emitter, galvo (rotates the mirror it’s facing when powered with a beam), splitter, mirror, delay, wall, and detector.

![Tiles](https://github.com/user-attachments/assets/be140531-560b-491c-be93-ba8bf985dddd)

## Controls

Use WASD or middle mouse + drag to pan and scroll to zoom.

| Key              | Action                                      |
| ---------------- | ------------------------------------------- |
| <kbd>Q</kbd>     | Copy hovered tile into cursor               |
| <kbd>E</kbd>     | Toggle state of the hovered emitter         |
| <kbd>R</kbd>     | Rotates the tile to selection in the cursor |
| <kbd>V</kbd>     | Flip vertically                             |
| <kbd>H</kbd>     | Flip horizontally                           |
| <kbd>P</kbd>     | Start (play) simulation                     |
| <kbd>T</kbd>     | Run cases (when in campaign level)          |
| <kbd>Space</kbd> | Run one simulation step                     |
| <kbd>+</kbd>     | Increase simulation speed                   |
| <kbd>-</kbd>     | Decrease simulation speed                   |
| <kbd>0</kbd>     | Reset simulation speed                      |
