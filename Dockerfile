FROM node:25-trixie AS node-builder

WORKDIR /app
COPY . .

RUN npm install

FROM rust:trixie AS rust-builder

WORKDIR /app
COPY --from=node-builder /app/ /app/

RUN rustup toolchain install stable && \
    rustup target add wasm32-unknown-unknown
RUN cargo install dioxus-cli

RUN dx bundle --release --platform web --debug-symbols=false

FROM nginx:stable-trixie AS result

COPY --from=rust-builder /app/target/dx/ev_calculator/release/web/public/ /usr/share/nginx/html/

CMD ["nginx", "-g", "daemon off;"]
