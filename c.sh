alias c='_c(){ cargo build --bin "$1"; cargo run --bin "$1";}; _c'
c "${1}"
