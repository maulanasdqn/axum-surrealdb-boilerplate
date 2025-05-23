# Axum SurrealDB StarterKit

## Features

- **Authentication Ready**: Preconfigured authentication and middleware for secure API access.
- **Database Integration**: SurrealDB seamlessly integrated as an Axum Extension.
- **CORS Handling**: Fine-tuned CORS management with Tower HTTP `CorsLayer`.
- **API Documentation**: Fully documented with OpenAPI and Swagger UI.
- **Optimized for Performance**: Asynchronous, lightweight, and scalable architecture.

## Prerequisites

- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **Database**: Set up a SurrealDB instance and configure connection details.
- **Docker**: Required for containerized deployment, install from [docker.com](https://www.docker.com/).
- **Nix (Optional)**: For reproducible builds, install from [nixos.org](https://nixos.org/).

## Getting Started

1. **Clone the Repository**:

   - `git clone https://github.com/maulanasdqn/axum-starterkit.git`

2. **Set Up Environment Variables**:

   - Copy `.env.example` and rename it to `.env`

   - **Windows**: Run the script: `./apply-env.ps1`
   - **Unix-based systems (Linux, macOS, BSD)**: Run the script: `source ./apply-env.sh`

3. **Install Dependencies**:

   - `cargo fetch`

4. **Setup Database**:

   - Install the surrealDB
   - **Windows**: `iwr https://windows.surrealdb.com -useb | iex`
   - **Unix-based systems (Linux, macOS, BSD)**: `curl -sSf https://install.surrealdb.com | sh`
   - Start the database `surreal start --user root --pass root`

5. **Start the Server**:

   - Install Cargo Watch `cargo install cargo-watch`
   - Run it with cargo watch `cargo watch -x "run --bin api"`

   The API will be available at `http://localhost:3000/docs`.

## Docker

1. **Build the Docker Image**:

   - `docker build -t axum-starterkit .`

2. **Run the Docker Container**:

   - `docker run --env-file .env -p 3000:3000 axum-starterkit`

## Using Nix as Builder (Optional)

1. **Install Nix**:

- Install Nix from [nixos.org](https://nixos.org/).

2. **Enter Nix Shell or Use Nix Flakes**:

- Enter Nix shell: `nix develop`
- Use Nix Flakes: `nix build`

3. **Build the Project**:

- `nix build`

4. **Run the Server**:

- `nix run`

## Contributing

Contributions are welcome! Fork the repository and create a pull request with your improvements.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Axum](https://github.com/tokio-rs/axum)
- [SurrealDB](https://github.com/surrealdb/surrealdb)
