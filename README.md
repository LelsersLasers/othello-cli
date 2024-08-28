# othello-cli
othello-cli is a cli version of Othello (Reversi) written in Rust.
You can play against another player, the AI, or watch two AIs play each other.

https://user-images.githubusercontent.com/48894200/183278088-707948e6-55cf-4346-bb36-961a3c0321e8.mp4

crates.io link: https://crates.io/crates/othello-cli

```
>>> othello-cli help
Usage: othello-cli [options]
Options:
  h, help               Show this help message
  b, black              Set black to be controlled by the user
  w, white              Set white to be controlled by the user
  bc, black-color       Set the color of black to be a custom color
                          default: green
                          format: 'othello-cli black-color r g b' where r, g, and b are integers from 0-255 
  wc, white-color       Set the color of white to be a custom color
                          default: red
                          format: 'othello-cli white-color r g b' where r, g, and b are integers from 0-255 
  mc, marked-color      Set the color of the valid moves to be a custom color
                          default: cyan
                          format: 'othello-cli marked-color r g b' where r, g, and b are integers from 0-255
  bp, black-piece       Set the piece for black to be a custom character
                          default: X
                          format: 'othello-cli black-piece c' where c is a single character
  wp, white-piece       Set the piece for white to be a custom character
                          default: O
                          format: 'othello-cli white-piece c' where c is a single character
  t, time               Set the milliseconds the AI waits before making a move
                          default: 750 ms
                          format: 'othello-cli time ms' where ms is a positive integer
```

Rules of Othello: https://www.eothello.com/#how-to-play

![Showcase Image 2](https://github.com/LelsersLasers/Othello/raw/main/Showcase/Showcase2.PNG)

![Showcase Image 1](https://github.com/LelsersLasers/Othello/raw/main/Showcase/Showcase.PNG)
