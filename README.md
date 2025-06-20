# Pitched

Pitch training app/game.

For now there is no customization, it is just single octave from c3..c4.

Run with midi port and type the notes you hear. You can exit by typing `q`.
```shell
pitched -p 128:0
```
![image](https://github.com/user-attachments/assets/88559522-b558-44f4-8d88-764da0008862)

This program uses midi signals. You need to use something that can play them.
For example [fluidsynth](https://www.fluidsynth.org/):
```shell
fluidsynth --server
```
