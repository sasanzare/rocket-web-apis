# rocket web apis


Written in Rust, and powered by Rocket and React.

## Cloning and building

### Backend

The backend of the blog is written in Rust
```
# Clone repository
git clone https://github.com/sasanzare/rocket-web-apis
# cd into the cloned repository
cd rocket-web-apis/backend-end

# Install diesel CLI (For database migrations)
cargo install diesel_cli
# Run migrations
diesel migrations run
# Build the server in release mode
cargo build --release
# Run the server
cargo run --release
```

### Frontend

The frontend of the API is written in React

```
cd rocket-web-apis/front-end

# Build the server
npm run build # With Npm
yarn build # With Yarn
# Run the server
npm run preview # with Npm
yarn preview # with Yarn
```
