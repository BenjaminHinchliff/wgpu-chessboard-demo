# WGpu Chessboard Rendering Demo

This is more of an experiment than anything else,
there is still a lot of room to improve this, including (but not limited to):

- [ ] make drawable region not mess up when resizing
- [ ] make it compatible with icu (or something to that effect)
- [ ] a little code cleanup

Feel free to try your hand at any of these if you want - I'm a little burned
out for the time being

## Compilation

Completely standard for a `cargo` project (`cargo run --release`), but to speed
up shaderc-sys compile time you might want to install `shaderc`
via whatever package manager you use.
