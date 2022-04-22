#!sh

rm /usr/local/bin/search

cargo build --release

echo "Copying binary to /usr/local/bin/search"
cp target/release/searcher /usr/local/bin/search
echo "search was installed"

