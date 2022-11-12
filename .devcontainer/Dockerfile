FROM docker.io/library/rust

RUN \
  --mount=type=cache,sharing=locked,target=/var/cache/apt/,id=ubuntu:22.04/var/cache/apt/ \
  --mount=type=cache,sharing=locked,target=/var/lib/apt/lists/,id=ubuntu:22.04/var/lib/apt/lists/ \
  mv /etc/apt/apt.conf.d/docker-clean /etc/apt/apt.conf.d/docker-gzip-indexes / \
  && apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends --no-install-suggests \
    sudo \
    vim \
  && mv /docker-clean /docker-gzip-indexes /etc/apt/apt.conf.d/

RUN useradd -mU -s /bin/bash -G sudo user \
    && echo %sudo ALL=\(ALL:ALL\) NOPASSWD:ALL > /etc/sudoers.d/nopasswd \
    && chmod 0440 /etc/sudoers.d/nopasswd

USER user