FROM secondstate/ssvm

ENV PATH $PATH:/root/.cargo/bin
RUN curl https://sh.rustup.rs -sSf | sh -s -- -v -y
RUN rustup default stable
RUN rustup update nightly \
    && rustup target add wasm32-unknown-unknown --toolchain nightly

RUN rm -rf /var/lib/apt/lists/*
