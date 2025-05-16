# Custom keyboard firmware
I recently build a keyboard hardware-wise mostly from scratch (setting up PCB, printing, assembling). I found it rewarding even if Ergogen took a lot of the guesswork out of it. But it occurred to me that I now, knowing the way keyboards work under the hood, I have everything I need to make a firmware from scratch.

## What this project is
This is a project for me to learn more about hardware programming in general, about embedded Rust in particular, and about keyboard firmware.

I will try my best to avoid copying ZMK/QMK, and haven't learned much about the actual implementation of either. But the basic functionality of a keyboard firmware is always the same, and this will result in a lot of similarities.

## Feature List
### Complete
None

### In Progress
- Well-designed config format 


## Configuration Format
To keep it simple I plan to do basic keymap design in a main file, and any extensions (custom behaviors, configuration) to a side file.

### Behaviors
For now I have chosen to take the ZMK terminology of "Behavior" as the general functionality assigned to a key in a layer. 

A behavior will be represented as an unseparated tuple. E.g. `(ht LCTRL ESC)` represents a hold-tap between Control and Escape. `(kp A)` is the A key.

I think that this strikes a good balance between keeping behavior specs visually separated, and limiting non-meaningful characters

### Layers
As with all keyboard firmware I've seen, this one will use layers.

The syntax for one layer for a 2x2 keyboard, named `BASE`, might be:
```
BASE: [
    (kp A) (kp B)
    (kp C) (kp C)
]
```

White space is ignored except for token separation.

### Variables
Sometimes it can be useful to define shorthands for some long-named behaviors, or keys. To facilitate this, there are two different types of variables:

#### Key Variables
These variables bind a keycode to another name, and it can then be used in the same place as any name. So, if you find yourself using the `ESC` key a lot, you can do something like `e: ESC`.

#### Behavior Variables
These variables bind a behavior to another name, and it can then be used in a layer definition by enclosing it with parentheses. For example, if you use a bare `(kp ESC)` binding a lot, you could do `e: (kp ESC)` and then use it in a layer definition as `(e)`

### Example
```
config: {
    tapping_term_ms: 150,    # Hold action triggers on holding for 150ms
};

variables: {
    sc_1: (ht LSHFT A),      # LSHFT when held, A when tapped
    sc_2: (ml NUM),          # go to NUM layer while held
};

layers: {
    BASE: [
        (sc_1) (kp B)
        (kc C) (sc_2)
    ],
    NUM: [
        (kp 1) (kp 2)
        (kp 3) (kp 4)
    ]
};
```

Layers is required, the others are optional.
