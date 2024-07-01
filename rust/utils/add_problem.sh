cd $(git rev-parse --show-toplevel)/rust/src/
touch solutions/$1.rs
touch tests/$1.rs
echo "pub mod $1;" >> solutions/mod.rs
echo "pub mod $1;" >> tests/mod.rs
cd -
