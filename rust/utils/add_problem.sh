cd $(git rev-parse --show-toplevel)/rust/src/

touch solutions/p$1.rs
touch tests/p$1.rs

echo "pub mod p$1;" >> solutions/mod.rs
echo "pub mod p$1;" >> tests/mod.rs

echo "pub struct Solution;" >> solutions/p$1.rs
echo "#[cfg(test)]
mod test {
    use crate::solutions::p$1::Solution;
}" >> tests/p$1.rs

cd -
