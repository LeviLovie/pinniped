[![](https://github.com/LeviLovie/pinniped/actions/workflows/test.yaml/badge.svg)](https://github.com/LeviLovie/pinniped/actions)

# pinniped
Stack based programming language

## Running
### Provided binaries
I build the project for Windows, MacOS, and Linux every time a new version comes out. You can download the lates binaries from the [releases](https://github.com/LeviLovie/pinniped/releases/latest).

### Compiling
You can compile the code yourself by downloading the source code from the [latest release](https://github.com/LeviLovie/pinniped/releases/latest) or from the [repo itself](https://github.com/LeviLovie/pinniped).

Please make sure [rust is installed](https://www.rust-lang.org/tools/install)

To compile run:
```sh
cargo build --release
```
That will generate `target/release/panniped` - that is your executable.

### Compiling the same as in [releases](https://github.com/LeviLovie/pinniped/releases)
In order to compile the same executables as in release you can run the build script:
```sh
./build.sh
```
> [!WARNING]
> This script only works on MacOS and requires [rust](https://www.rust-lang.org/tools/install) and [docker](https://www.docker.com)
