# Progressive examples

this directory will contain examples that progressively add features.

The features are in order:

1. `minimum_window`: Just displays a window with a background color
2. `img_backend`: Uses an image backend to create a png file from the game loop.
3. `multiple_backends`: Shows that we can use our same game loop with multiple backends. ie: shows off code reuse.
4. `simple_grid`: Shows that we can use the Canvas api in `src/lib.rs` to draw lines
5. `tilted_grid`: Shows logic of printing diagonal tiles.
6. `tiled_map`: Expands on `tilted_grid` to move code into seperate functions, and renders on top of an abstraction of a game map that transforms the map coordinates to screen coordinates.
7. `events`: Shows that we can process events (only applicable to certain backends obviously)
8. `event_state`: Shows that we can keep track of our own state in response to events we see. For example, we can always know the current position of the mouse so that we can use that in the draw function instead of having to ask the backend to track that for us.
9. `tiled_map_select`: Shows that we can properly map back and forth between map and screen coordinates. a red box should appear around the tile you have selected.
10. `pan`: Temporarily going away from isometric stuff to work on panning and zooming. It was easier to implement this while looking at less code. This implements a simple panning functionality. Inspired by: https://www.youtube.com/watch?v=ZQ8qtAizis4
11. `pan_and_zoom`: More advanced than pan. Quite a bit more code. Also borrowed from the above mentioned tutorial. Right click anywhere to draw a diagonal red line which should keep its relative position to the grid as it is panned and zoomed.
12. `pan_and_zoom_world_screen`: The previous pan and zoom example is a bit unwieldly. Theres too much code, and the logic of panning/zooming is mixed with the logic of handling state/drawing. We split out the panning/zooming logic into a seperate struct, and then this `pan_and_zoom_world_screen` example leverages that to achieve the same functionality but with less logic in the 'game' code.
