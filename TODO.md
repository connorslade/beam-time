# To Do

## Engine

- [x] Cleanup sprite pipeline
- [x] Support multiple atlases
- [x] Text rendering
- [x] Convert all positions to floats
- [x] Should scale be automatically multiplied by scale_factor if position cant?
- [x] Supply all four points in GpuSprite
- [x] Allow rotating sprites
- [ ] ~~Look into anti aliasing~~
- [ ] ~~Matrix stacks?~~
- [x] Color struct with mixing and lab
- [x] Sprite Z-ordering
- [x] Bring back the input handler for immediate mode style ui layouts
- [x] Some kinda screen that always runs its handlers
- [x] Build out text renderer
- [x] Fix scale anchor bug
- [x] Strongly typed asset references
- [ ] ~~Figure out how to apply custom shader effects?~~
- [ ] ~~Cleanup coordinate systems~~
- [x] Improve FPS - Basically optimize sprite pipeline
  - [x] Use instancing instead of making huge meshes
  - [ ] Also avoid re-creating buffers or bind groups every frame
- [ ] Engine key chord pressed
- [x] Allow canceling clicks through the graphics context
- [ ] ~~Cleanup public fields in engine that should be read only~~
  - Its not that big of a deal
- [x] Extend engine with support for clipping sprites to some area
  - Use instancing of a quad mesh with a uniform containing position, color, depth, clip bounds, etc.
- [x] Solid shape renderers (polygon)
- [ ] Use a buffer binding array instead of dispatching the sprite render for each texture atlas?
- [x] Update draw_callback with support for polygons
- [x] Update sprite renderer to make use of the bytemuck impl for nalgebra
- [x] Remove screen system to take out App generic from graphics context
  - [x] Add on resize event to graphics context
- [x] Allow setting a `dynamic_scale` on sprites / drawables to allow for a scale anchor
- [x] Implement layout system
  - [ ] Cleanup with martix stacks?
  - [ ] Add justify support to column and row layouts
  - [ ] Figure out how to check if hovered
    - A 'positioned element' type
    - Wrapper type that caches is_hovered after layout run for the next frame

## User Interface

- [x] Make a button UI element
- [x] In the background of the title screen have faded versions of all the tiles be flying down
- [x] Fade buttons to accent color on hover
- [x] Create z_order consts for layers
- [x] Sandbox save selection screen
- [x] Verify ui scaling
  - [x] Add debug hotkey to change between 1x and 2x scale
  - [x] Fix ui scaling of rectangle outline
- [ ] Figure out ui scale config option...

## Beam Logic

- [x] Implement beam logic
- [x] Fix galvo input logic
- [x] Dont overwrite galvo mirror state when rotating
- [x] Actual Infante Boards
  - [x] Use new rendering system
  - [x] Use hash map for tile storage
  - [x] Destroy beams after traveling too far
- [ ] Fix simulation bug when two galvos are pointed at the same mirror
- [ ] Splitters take input from all sides to avoid non-deterministic behavior when used as or-gates
- [x] Split beam logic into its own crate (so solutions can be verified on the server)
- [x] Cleanup how level results are handled
  - This also includes fixing the bug where the simulation thread can crash if the tick speed is too high
- [x] Figure out what to use as user IDs to prevent multiple score uploads for one player
  - [x] Implement custom hardware id
- [x] Set LevelMeta::solved to true when level is solved and unset it on board modification
  - [x] Fire event on board edit
- [ ] Fix non-deterministic beam logic
  - [ ] Create test cases to verify simulation
- [x] Fix edge detector levels
- [ ] Load custom levels at runtime (on server too :sob:)
- [x] Fix synchronization level checker
- [x] On level result animation, use previous price instead of current
- [ ] Make emitters default off in levels?
- [x] Allow checking powered status of non detector tiles in level checkers
- [ ] Allow changing default values of level emitters

## Misc

- [x] Figure out how to pass an app struct around
- [x] Split tile_map and tile_picker into ui/game components
- [ ] ~~Persist window size~~
- [ ] ~~Experiment with animation icon?~~
  - This is completely unnecessary, but might be kinda cool, maybe an option in settings
- [x] Fix rpath on linux
- [x] Use ahash HashMap and HashSet

## Game

- [x] Saves should have a header at the start
  - Save name
  - Playtime
  - Last open date
- [x] Autosave interval
- [ ] ~~Cleanup history track_once vs track_many~~
- [x] Allow changing simulation speed
  - [x] Decouple simulation from renderer
- [ ] Start testing at currently open level panel test case, looping around at the end
- [x] Allow disabling tiles in level def
- [x] Price system
- [ ] ~~Use ron for config file?~~
  - Just to remove toml dep if its only used once
- [x] Add a solution latency metric
- [x] Split beam_time::game::board into some sub modules
- [x] Put steam features under a feature flag
- [x] Cache get_results stuff
- [ ] Figure out how to support user levels on leaderboard server (steam workshop)
- [ ] Fix screen_to_world_space and world_to_screen_space
- [ ] Use screen_to_world_space to handle tile interactions because calling .is_hovered 50k times per frame is kinda slow

### Rendering

- [x] Fix delay gate rendering z order
- [x] Render the little light pixel on active emitters
- [x] Cleanup selection renderer with holes

### Interface

- [x] Copy & paste tiles
  - [x] Allow making selections
  - [x] Allow adding and subtracting from selections
  - [x] Add some selection operations
    - [x] Delete
    - [x] Duplicate into holding
    - [x] Move into holding
- [x] Undo
  - [x] Store list of modifications
- [x] Don't overstep pan/zoom goals during lag spikes
- [ ] Allow managing (creating, deleting) sandbox saves
- [x] Zoom around the cursor position
- [x] Stop simulation on delete, cut, or copy
- [x] Pop screen & save/free simulation on ESC
- [x] Allow rotating copied selections
- [x] Visual feedback for test cases
- [ ] Blueprint system
  - [ ] Allow saving and loading premade-structures in-game
  - [ ] Allow importing and exporting them as base64 encoded strs
- [x] Dont overwrite when placing tile normally
- [ ] Dont allow interacting with protected tiles through selections
- [x] Use shift to reverse rotation direction
- [ ] ~~Bias scale to integer zoom levels when at small zooms~~
- [x] Detect and show failed test cases instead of hanging
- [x] Allow labeling emitters / outputs / tiles in general?
- [x] Show current test case in level panel when simulating in test mode
- [x] Render histograms on level completion screen
- [ ] Take a look at test case viewer for Double It level
- [ ] Fix up histograms
  - [ ] Render correctly if all bins are the same / zero
  - [ ] Render where you fall more correctly
  - [ ] Emulate a smaller number of bins by repeating
- [x] Fix input/output label rendering for test cases
  - [x] Add output label support
  - [x] Add labels to half adder level
- [x] Decide if decay or linier interpolation looks better for tile picker
- [x] When making a selection show its size and price
- [x] Scroll faster at lower zoom levels
- [ ] Fix weird animation in level info panel when making screen much larger than the previous frame
- [ ] Add sticky notes with text
- [ ] Fix scroll speed difference between track pad and mousewheel
- [ ] ~~Use graph layout for campaign?~~
- [ ] Little keyboard shortcut helper thing on bottom right of screen when in game
- [ ] Use column layout for level panel
- [ ] Setting model on title page
- [ ] Paused modal in game

---

## Queue

- Test case renderer for 'Double It'
- Keyboard shortcut helper
- Ui buttons for sandbox modal
- Fix world space ←→ screen space conversion functions
  - To make testing in debug mode possible
- Paused modal in game
