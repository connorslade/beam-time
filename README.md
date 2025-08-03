<img src="https://github.com/user-attachments/assets/7a7d112f-d1fe-4ebc-9871-f0752f28a168" />

---

Beam time is a logic puzzle game where you redirect and split laser beams to create digital circuits. Through the campaign, you will explore logic gates, oscillators, latches, counters, adders, memory, and more.
Each level provides a description and some test cases your solution must pass.
Inputs are passed in through laser emitters and outputs are made with beam detectors.
It's your job to use some combination of mirrors, beam splitters, delay gates, and galvos to complete the level. Galvos are the only 'active' component, they rotate the mirror they are facing when powered.

![beam-time](https://github.com/user-attachments/assets/0c7e36e2-f215-4d70-94a4-d23da675fd51)

Here are all seven in-game components from left to right: emitter, galvo (rotates the mirror it’s facing when powered with a beam), splitter, mirror, delay, wall, detector. With just these few components, It's possible to perform any computation.

![Tiles](https://github.com/user-attachments/assets/5a59ce18-30b6-4d62-b6ce-6b04fc648a01)

The game also features a sandbox mode where you can build whatever you want.
Below is an implementation of the one-dimensional elementary cellular automaton [Rule 30](https://en.wikipedia.org/wiki/Rule_30).
In the future I want to build a programmable computer within the sandbox.

![beam_time_USDkpuxkMO](https://github.com/user-attachments/assets/cdd9012a-4f53-487b-9c96-7de6b85262ec)

<details>
<summary>Controls</summary>

Use WASD or middle mouse + drag to pan and scroll to zoom.

| Key                           | Action                                          |
| ----------------------------- | ----------------------------------------------- |
| <kbd>Q</kbd>                  | Copy hovered tile into cursor                   |
| <kbd>E</kbd>                  | Toggle state of the hovered emitter             |
| <kbd>R</kbd>                  | Rotates the tile to selection in the cursor     |
| <kbd>Shift</kbd>+<kbd>R</kbd> | Same as above but in the opposite direction     |
| <kbd>V</kbd>                  | Flip vertically                                 |
| <kbd>H</kbd>                  | Flip horizontally                               |
| <kbd>1</kbd>-<kbd>7</kbd>     | Picks up tile from panel                        |
| <kbd>Shift</kbd>+drag         | Make a selection (ctrl to add, alt to subtract) |
| <kbd>U</kbd>                  | Delete current selection                        |
| <kbd>N</kbd>                  | Create sticky note at mouse position            |
| <kbd>Ctrl</kbd>+<kbd>Z</kbd>  | Undo                                            |
| <kbd>P</kbd>                  | Start (play) simulation                         |
| <kbd>T</kbd>                  | Run cases (when in campaign level)              |
| <kbd>Space</kbd>              | Run one simulation step                         |
| <kbd>+</kbd>                  | Increase simulation speed                       |
| <kbd>-</kbd>                  | Decrease simulation speed                       |
| <kbd>0</kbd>                  | Reset simulation speed                          |
| <kbd>Shift</kbd>+<kbd>0</kbd> | Max simulation speed                            |

</details>

<details>
<summary>Publishing to Steam</summary>

I have already forgotten how to do this like twice since starting this project, so I'm just going to write it down for next time...

In the steamworks app data admin page, go to steam pipe › builds then click on 'show older builds'.
This will give the option to upload builds from a zip.
The zip should contain the beam_time executable along with the shared library for the current platform.

</details>
