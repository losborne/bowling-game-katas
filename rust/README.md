1. Install dependencies
  ```
  (sudo) apt install curl git build-essential 
  ```

2. Install rustup
  ```
  curl https://sh.rustup.rs -sSf | sh
  ```

  Select 2) Customize installation.
  Press enter to select default host triple.
  Type nightly to select nightly default toolchain.
  Press enter to select default for modify PATH variable.

  Type 1 to proceed with installation.

  Add the newly installed rust to your PATH.
  ```
  source $HOME/.cargo/env
  ```

  Set rustup to use the nightly toolchain by default
  ```
  rustup default nightly
  ```

3. Clone this repository and go to the rust directory.

  ```
  git clone https://github.com/losborne/bowling-game-katas
  cd bowling-game-katas/rust
  ```

4. Run the tests
  ```
  cargo test
  ```
