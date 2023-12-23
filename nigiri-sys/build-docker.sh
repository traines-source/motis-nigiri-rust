SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
docker run -v ${SCRIPT_DIR}:/app/ -v ${HOME}/.ssh/:/home/build/.ssh/:ro \
--rm -it $(docker build -q .) \
/bin/bash ./build.sh