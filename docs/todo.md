# TODO

## Base Functionality

DONE!

This is what's left for a feature complete MVP. 

- [X] Export image
- [X] Save album/project
- [X] Import images
- [X] Linear mask

## Refinement

- [ ] Crop UX. It's not very intuitive right now.
- [ ] Ensure crop can't be outside image.
- [X] Parameter formulas. They don't look very good right now.
- [ ] Highlights/shadows
- [X] Radial mask width and height
- [X] Radial mask rotation
- [ ] Radial mask feathering
- [ ] Display of outside the image (texture artifacts)
- [ ] Prevent artifacts during exports (lines when exporting weird aspect ratios)
- [ ] Undo/redo
- [ ] Auto save
- [ ] General UI improvements

## Bugs

- [X] When switching image while having a radial mask active, there's risk of index out of bounds

## Performance

- [ ] Load photos in background thread so it doesn't hog the entire application
- [ ] Don't reload entire album when import new image