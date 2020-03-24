# simple_sprite_animations

Simple sprite animation example to help myself learn how to use Amethyst game engine.

✔️ Idle animation
✔️ Walking Animation
🚧 Hit animation

## How to run

To run the game, use

```
cargo run --features "vulkan"
```

on Windows and Linux, and

```
cargo run --features "metal"
```

on macOS.

For building without any graphics backend, you can use

```
cargo run --features "empty"
```

but be aware that as soon as you need any rendering you won't be able to run your game when using
the `empty` feature.

## Resources
Spritesheet from https://0x72.itch.io/dungeontileset-ii
