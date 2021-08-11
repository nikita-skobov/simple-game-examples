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
