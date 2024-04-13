SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
docker run -v ${SCRIPT_DIR}:/app/ \
--rm -it $(docker build -q ./nigiri-sys) \
/bin/bash -c "cargo test"