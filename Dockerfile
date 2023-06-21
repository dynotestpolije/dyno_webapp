# ==========================================================================================#
# menggunakan builder sebagai image builder dan menyalin hasil dari build pada image builder ke
# image yang akan digunakan sebagai kontainer yang digunakan sebagai aplikasi
FROM rust:latest as builder

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /dynotests

COPY . .

RUN cd ./frontend && trunk build --release && cd /dynotests

RUN RUSTFLAGS="-C target-cpu=native" cargo build --bin backend --release
# mengambil linked library yang terhubung oleh apliaksi dengan ldd 
RUN ldd /dynotests/target/release/backend | tr -s '[:blank:]' '\n' | grep '^/' | xargs -I % sh -c 'mkdir -p $(dirname deps%); cp % deps%;'


# ==========================================================================================#
FROM scratch 
# mengambil hasil dari executable rust hasil compile
COPY --from=builder /dynotests/deps /
COPY --from=builder /dynotests/target/release/backend /bin/dyno_server

WORKDIR /dynotests
COPY --from=builder /dynotests/public .
COPY --from=builder /dynotests/.env .
COPY --from=builder /dynotests/sqlite:dynotest_database.db .

# Eksekusi Rust Backend Service
ENTRYPOINT ["/bin/dyno_server"]
# ==========================================================================================#
