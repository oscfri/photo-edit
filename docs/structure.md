# Structure

## Terminology

When I started developing this I found myself coming up with similar names for different concepts in the program. This is an attempt to sort things out.

### User Interface

The purpose of the user interface is to act as an interaction point to the work space.

There are the following distinctions:

- Window
- Pane
- Component

A window is the main container. It contains all components that should be displayed. Only one window can be active at a time.

A container displays one aspect of the current workspace.

The user interface has the following elements:

#### Windows

- WorkspaceWindow

Potential windows in the future:

- AlbumSelectionWindow

#### Panes

A pane is a collection of components. It divides the window into multiple parts.

- ToolBoxPane: this is where parameters and other image manipulation elements are shown
- RenderPane: this is where the image is rendered. More on the view port later.
- ImageSelectionPane: this is where the image selection pane is displayed.

#### Components

A components shows one aspect of a pane.

- ViewPortComponent: this is where the image is shown

### Work space

The work space is where everything is happening.

- WorkSpace