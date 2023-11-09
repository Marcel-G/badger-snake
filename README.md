# Badger Snake

Snake for the Badger 2040.

https://github.com/Marcel-G/badger-snake/assets/2770666/2da3852e-9e3a-4144-be81-385068359fb1

Started from the [rp2040-project-template](https://github.com/rp-rs/rp2040-project-template) see for setup instructions.

## Run the simulator

```bash
cargo run --bin simulator --features=graphics-simulator
```

## Build and upload to Badger 2040

```bash
cargo run --bin badger_2040 --features=rp2040-hal --target=thumbv6m-none-eabi
```
