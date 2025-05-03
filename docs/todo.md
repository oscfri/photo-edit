# TODO

## Base Functionality

DONE!

This is what's left for a feature complete MVP. 

- [X] Export image
- [X] Save album/project
- [X] Import images
- [X] Linear mask

## Refinement

- [X] Crop presets (4:3, 16:9, etc.)
- [X] Radial mask feathering
- [X] Prevent artifacts during exports (lines when exporting weird aspect ratios)
- [ ] General UI improvements
- [X] Parameter formulas. They don't look very good right now.
- [X] Radial mask width and height
- [X] Radial mask rotation
- [X] Display of outside the image (texture artifacts)
- [X] Undo/redo
- [X] Auto save
- [X] Flag/filter photos
- [X] Show/hide applied parameters
- [X] Disable parameters (for example during image load)
- [X] Move database to proper config path (using directories crate)

## Bugs

- [ ] Filter with no favorite photos
- [ ] Toggle favorite when filter is active
- [ ] Ensure exports folder exists
- [X] When switching image while having a radial mask active, there's risk of index out of bounds

## Performance

- [X] Save thumbnails in database
- [X] Don't have all images loaded in memory
- [ ] Export photos in background
- [X] Load photos in background thread so it doesn't hog the entire application
- [X] Don't reload entire album when import new image

## Bonus Refinement

- [ ] Crop UX. It's not very intuitive right now.
- [ ] Ensure crop can't be outside image.
- [ ] Batch export
- [ ] Set exports directory
- [X] Highlights/shadows