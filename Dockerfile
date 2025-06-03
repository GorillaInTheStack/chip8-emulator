FROM debian:12-slim

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
  xfce4 \
  xfce4-terminal \
  tigervnc-standalone-server \
  tigervnc-tools \
  novnc \
  websockify \
  dbus-x11 \
  curl \
  build-essential \
  ca-certificates \
  sudo \
  libsdl2-dev \
  libsdl2-image-dev \
  libsdl2-ttf-dev \
  && rm -rf /var/lib/apt/lists/*

RUN useradd -m developer \
  && echo "developer ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

USER developer
ENV HOME=/home/developer
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y \
  && . "$HOME/.cargo/env" \
  && rustup update stable

COPY --chown=developer:developer entrypoint.sh /home/developer/entrypoint.sh
RUN chmod +x /home/developer/entrypoint.sh

EXPOSE 5901 6080

CMD ["/home/developer/entrypoint.sh"]

