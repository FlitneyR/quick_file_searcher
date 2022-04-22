install_path="/usr/local/bin/search"

rm $install_path

cargo build --release

echo "Copying binary to /usr/local/bin/search"
sudo cp target/release/searcher $install_path
echo "search was installed"

