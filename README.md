# conway
Grid based simulation in rust  
  
This is an attempt to
* Implement an understood and pre-existing program in idiomatic rust style 
* Provide neighbor-determination and state change conditions as abstractions through closures
* Experiment with animation techniques

The hope is to get more familiar with rust while creating a modular grid-based simulation and visualization engine.
The engine happens to default to Conway's Game of Life.

---
## Installation/Compilation notes
Install rust:
`https://www.rust-lang.org/en-US/install.html`

Download conway's source code:
`git clone https://github.com/JeremyCooper/conway`
`cd conway`

Build and install conway:
`cargo install --features="binaries"`

If all went a compiled binary should have been placed in `~/.cargo/bin`.  
It is recommended to add that directory to your $PATH variable.

---
## Manual

# Controls:

`space`: pause/play
`n`: step forward
`q`: quit


# Command-Line options:

`-i, --initial-state`: Initial grid state.  
    Possible values:  
        Random (default)  
        LWSS  
        Glider  
        Eureka  
        Pulsar  

`-a, --input-cells`: Cells to consider adjacent to cell currently being evaluated.  
    Possible values:  
        Neighbors (default)  
        NoCorners  
        CornersOnly  
        FarOnly  
        FarAlso  
        FarCornersOnly  

`-r, --ruleset`: Ruleset to use to determine cell state changes.  
    Possible values:  
        Conway (default)  
        ConwayEasy  
        ConwayVeryEasy  
        Decay  

`-t, --delay`: Delay between frames, in milliseconds

`-w, --width`: Width of grid, in cells

`-h, --height`: Height of grid, in cells

`-l, --live-char`: Character for displaying live cells

`-d, --dead-char`: Character for displaying dead cells

`-c, --color`: Color for live cells

`-b, --dead-color`: Color for dead cells

`-f, --filled`: In place of characters, fill entire cell for display

`-p, --padding-off`: Do not pad cells

`-v, --inverse`: Reverse live and dead cell options for display

`-z, --time-slice`: Cells drift upwards

# Examles:

`$ conway-cli -l 'r' -d 'j' -b Red -c Blue`

`$ conway-cli -t 25 -c Cyan -b Red -l 'O' -d '|' -r Decay`

`$ conway-cli -t 25 -c Green -a NoCorners -f -r ConwayVeryEasy`
