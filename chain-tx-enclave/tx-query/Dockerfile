FROM baiduxlab/sgx-rust:1804-1.0.9
LABEL maintainer="Crypto.com"

RUN echo 'source /opt/sgxsdk/environment' >> /root/.docker_bashrc && \
    echo 'source /root/.cargo/env' >> /root/.docker_bashrc

RUN apt-get update && \
    apt-get install -y --no-install-recommends libzmq3-dev && \
    rm -rf /var/lib/apt/lists/*

ARG SGX_MODE=HW
ARG NETWORK_ID

ENV TX_VALIDATION_CONN=${TX_VALIDATION_CONN}
ENV SPID=${SPID}
ENV IAS_API_KEY=${IAS_API_KEY}
ENV TX_QUERY_TIMEOUT=${TX_QUERY_TIMEOUT}

ENV APP_PORT=25944

COPY . .

RUN ./chain-tx-enclave/tx-query/make.sh

WORKDIR /root/chain-tx-enclave/tx-query/bin

CMD ["../entrypoint.sh"]
