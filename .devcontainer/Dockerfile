FROM mcr.microsoft.com/devcontainers/rust:1-1

# Install dotfiles
RUN git clone https://github.com/fffclaypool/setup.git /home/vscode/dotfiles \
    && chmod +x /home/vscode/dotfiles/install.sh \
    && /home/vscode/dotfiles/install.sh

# Install gcloud cli
RUN curl https://dl.google.com/dl/cloudsdk/channels/rapid/downloads/google-cloud-cli-447.0.0-linux-x86_64.tar.gz \
        -o /home/vscode/google-cloud-cli-447.0.0-linux-x86_64.tar.gz \
    && tar -xf /home/vscode/google-cloud-cli-447.0.0-linux-x86_64.tar.gz -C /home/vscode \
    && rm /home/vscode/google-cloud-cli-447.0.0-linux-x86_64.tar.gz \
    && echo source /home/vscode/google-cloud-sdk/completion.bash.inc >> /home/vscode/.bashrc \
    && echo source /home/vscode/google-cloud-sdk/path.bash.inc >> /home/vscode/.bashrc

# Install shfmt
RUN curl -Ssl https://github.com/mvdan/sh/releases/download/v3.2.4/shfmt_v3.2.4_linux_amd64 -o shfmt \
    && mv shfmt /usr/local/bin \
    && chown vscode /usr/local/bin/shfmt \
    && chmod a+x /usr/local/bin/shfmt

# Install network tool
RUN apt-get update \
    && apt install -y iptables iptables-persistent ethtool
