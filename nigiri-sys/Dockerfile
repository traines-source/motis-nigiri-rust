FROM ghcr.io/motis-project/docker-cpp-build

RUN apt-get update && apt-get install -y curl openssh-client

#RUN useradd -m -s /bin/bash build 
#USER build
USER ubuntu

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN /home/ubuntu/.cargo/bin/cargo install bindgen-cli

ENV PATH=/home/ubuntu/.cargo/bin/:$PATH
WORKDIR /app

#RUN mkdir ~/.ssh && ssh-keyscan -H github.com >> ~/.ssh/known_hosts

CMD ["cmake", "--version"]