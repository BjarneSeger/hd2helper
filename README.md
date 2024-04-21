# hd2helper
Learn the keybinds for the Stratagems in Helldivers 2, and maybe some other useful things in the future.

# Running
This is currently not packaged for any distro, but a flatpak is in the works. Until then, you will have to build this from source.
I *highly* recommend using the nix flake, as this ensures you have all dependencies installed and on the same version as me. Simply type
```
nix develop
```
and then
```
cargo run --release
```
and let it build. Precompiled releases will happen as soon as I figure out Github Actions.