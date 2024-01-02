set -xe
git clone https://github.com/motis-project/nigiri.git || echo "Skipping cloning, directory already exists."
mkdir -p nigiri/build/
cd nigiri/build/
cmake -DCMAKE_BUILD_TYPE=Release -GNinja ..
ninja -v 2>&1 | grep nigiri-server | tr " " "\n" | grep lib > nigiri_deps.txt
./nigiri-test
#./nigiri-server ../../tests/fixtures/gtfs_minimal_swiss/
cd ..
bindgen -o ../src/bindings.rs include/nigiri/abi.h
#bindgen -o ../src/bindings.rs  --opaque-type std::* --opaque-type 'cista::*' --opaque-type 'location_idx_t::*' include/nigiri/abi.h -- -x c++ -std=c++20 -I include/ $(printf -- '-I %q ' deps/*/include/)
cd ..
cargo test -- --nocapture