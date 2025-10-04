# termlogo

Super simple little terminal logo for some ascii fun

This is a testing repo for some things I was experimenting with. Enjoy

---

##  Features

-  Fully ASCII (no Unicode or special characters)
-  Cross-platform: works on Linux, Windows, macOS, and more
-  No dependencies
-  Testable and portable
-  Useful for terminal banners, debug markers, CI/CD output, or just fun

---

## Usage

```bash
termlogo "YOUR MESSAGE HERE"
```

### Example
```bash
termlogo "HELLO TEST"
```
This prints a large ASCII banner version of your message to the terminal.

Output:
```
#  #  ####  #     #      ##         ##### ####   ###  ##### 
#  #  #     #     #     #  #          #   #     #       #   
####  ###   #     #     #  #          #   ###    ##     #   
#  #  #     #     #     #  #          #   #        #    #   
#  #  ####  ####  ####   ##           #   ####  ###     #   
```

### Options
```
-h, --help     Show help and usage info
```
Output:
```
TermLogo - Simple ASCII Logo Output Tool

Usage:
  termlogo.exe "YOUR TEXT HERE"
  termlogo "Another Message"

Options:
  -h, --help     Show this help message
```

## Building

* These building tips for building on macOS for other platforms

Linux ( Static Binary):
Run the following brew command:
```bash
brew install filosottile/musl-cross/musl-cross
```
Add the following entry to your `cargo/config.toml`
```bash
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```
This entry tells `cargo` to build using this linker when building for the target

Add the target to rust:
```bash
rustup target add x86_64-unknown-linux-musl
```
Build:
```bash
cargo build --release --target x86_64-unknown-linux-musl
```

Windows: 
Run the following brew command:
```bash
brew install mingw-w64
```
This will allow you to the build against the Windows target successfully.

Add the following entry to your `cargo/config.toml`
```bash
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
```
This will tell `cargo` to use that linker when building using the Windows target

Add target
```shell
rustup target add x86_64-pc-windows-gnu
```

Build:
```bash
cargo build --release --target x86_64-pc-windows-gnu
```
The exe file will be found here:
```
target/x86_64-pc-windows-gnu/release/termlogo.exe
```

