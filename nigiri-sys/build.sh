set -xe
git clone https://github.com/motis-project/nigiri.git || echo "Skipping cloning, directory already exists."
mkdir -p nigiri/build/

cd nigiri/
cmake -G Ninja -S . -B build --preset=linux-amd64-release
#cmake -G Ninja -S . -B build --preset=clang-tidy
cmake --build build --target nigiri-test nigiri-import nigiri-benchmark -- -v | tee /dev/tty | grep nigiri-import | tr " " "\n" | grep lib | grep -v libc++ > build/nigiri_deps.txt

#cd nigiri/build/
#cmake -DCMAKE_BUILD_TYPE=Release -GNinja ..
#cmake --preset=clang-tidy -DCMAKE_BUILD_TYPE=Release -GNinja .. && cd clang-tidy && ninja && cd ..
#ninja -v | tee /dev/tty | grep nigiri-server | tr " " "\n" | grep lib > nigiri_deps.txt

./build/nigiri-test
#./build/nigiri-server ../tests/fixtures/gtfs_minimal_swiss/
#cd ..
bindgen -o ../src/bindings.rs include/nigiri/abi.h
#bindgen -o ../src/bindings.rs  --opaque-type std::* --opaque-type 'cista::*' --opaque-type 'location_idx_t::*' include/nigiri/abi.h -- -x c++ -std=c++20 -I include/ $(printf -- '-I %q ' deps/*/include/)
cd ..
cargo test -- --nocapture