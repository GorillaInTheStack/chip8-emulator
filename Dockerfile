FROM archlinux:base-devel

ARG STACK_PKGS=""
ENV HOSTNAME="devcontainer"

RUN useradd -m devuser && echo "devuser ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

RUN pacman -Syu --noconfirm \
    base-devel git curl neovim zsh starship sudo $STACK_PKGS \
    && pacman -Scc --noconfirm

RUN mkdir -p /home/devuser/.config

RUN git clone https://github.com/zsh-users/zsh-autosuggestions /usr/share/zsh/plugins/zsh-autosuggestions && \
    git clone https://github.com/zsh-users/zsh-syntax-highlighting /usr/share/zsh/plugins/zsh-syntax-highlighting

RUN mkdir -p /home/devuser/.config && \
    curl -fsSL https://gist.githubusercontent.com/GorillaInTheStack/40ace1e2b73c1aac67fad15441c7e7a5/raw/.zshrc -o /home/devuser/.zshrc && \
    curl -fsSL https://gist.githubusercontent.com/GorillaInTheStack/40ace1e2b73c1aac67fad15441c7e7a5/raw/starship.toml -o /home/devuser/.config/starship.toml

RUN chown -R devuser:devuser /home/devuser

USER devuser
WORKDIR /home/devuser/workspace

CMD ["zsh"]

