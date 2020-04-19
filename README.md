# Connect-Four

A recreation of the popular game of Connect-Four featuring an AI player with multiple difficulties. This web app was built purely using Rust and WebAssembly. We used [Rocket](https://rocket.rs/) as the backend framework, [Yew](https://yew.rs/docs/) as the frontend framework, and [Cargo-Web](https://github.com/koute/cargo-web) as the build tool. Check out this [sweet video](https://www.youtube.com/watch?v=4nV84WVmwwU) for more details!

# Get Started

### Install build dependencies

```bash
sudo apt install -y build-essential pkg-config libssl-dev
```

### Build the frontend

```bash
cargo install -f cargo-web
```

Go to the `connect-four-frontend` directory, then run: 

```bash
bash build.sh
```

### Install the database

Install and start MongoDB: https://www.mongodb.com/download-center/community

### Start the backend

```bash
rustup default nightly
```

Go to the `connect-four-backend` directory, then run:

```bash
cargo run
```

### See the result

Open the game at http://localhost:8000/

### Run the CLI

Go to the `connect-four-cli` directory, then run:

```bash
cargo run
```

# Built By

[Dinula](https://github.com/dinulade101), [Hugo](https://github.com/jspenguin2017) and [Kai](https://github.com/Kai-Bailey)
