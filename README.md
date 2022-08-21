# rust-in-game

Mostly a training project. The objective is simple: Create a web browser game with the least javascript as possible.

For this, I use rust + wasm. The http server serving the webpage should also be in rust (maybe will be in another project)


## Build and test

Requirement
 - rust

To build the project you need to install the dependencies
```sh
$ cargo install
```
Then build the project
```sh
$ wasm-pack build --target web
```

For testing purpose, you can then run
```sh
$ python3 -m http.server
```

## Roadmap

 - [x] Have a compiling code
 - [x] Be able to manipulate canvas
 - [x] Keyboard catching
 - [x] Scene manager
 - [ ] Global drawer
 - [ ] Physic engine (/!\ functional paradigm)
 - [ ] Design an actual game
  - [ ] Implement it
 - [ ] Play the game
 - [ ] Document the code
