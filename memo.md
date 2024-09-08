# memo on account of working under wsl2
  1. Using [Cross.rs](https://github.com/cross-rs/cross)
    - Building:
      `cross build --target x86_64-pc-windows-gnu`
    - Releasing:
      `cross build --release --target x86_64-pc-windows-gnu`
    
  2. Using WSL2's GUI
    - Running:
      `crun` (which is an alias in my .profile for `export WGPU_BACKEND=vulkan; cargo run`, b/c it fails w/o that ENV export)
    - Releasing:
      - run `./win-release.sh` in the project root 
