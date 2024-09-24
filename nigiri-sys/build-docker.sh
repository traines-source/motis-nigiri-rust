SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
# mount .ssh in order for cmake to be able to git clone nigiri's dependencies
docker run -v ${SCRIPT_DIR}:/app/ -v ${HOME}/.ssh/:/home/ubuntu/.ssh/:ro \
--rm -it $(docker build -q .) \
/bin/bash ./build.sh