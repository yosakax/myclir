FROM rust:1.70.0
LABEL maintainer="Yasuhiro OSAKA(fallingfluit.gmail.com)"

SHELL ["/bin/bash", "-c"]
ARG UNAME=docker
ARG UID=1000
ARG GID=1000

ENV DEBIAN_FRONTEND "noninteractive"
RUN apt-get update && apt-get upgrade -y && \
    apt-get install -y \
    locales \
    sudo \
    bash-completion
ENV TZ=Asia/Tokyo
RUN locale-gen ja_JP.UTF-8


## set locale
RUN localedef -f UTF-8 -i ja_JP ja_JP.UTF-8
ENV LANG="ja_JP.UTF-8" \
    LANGUAGE="ja_JP:ja" \
    LC_ALL="ja_JP.UTF-8" \
    TZ="Asia/Tokyo"



## create user
RUN useradd -m --uid ${UID} -d /home/${UNAME} --groups sudo  ${UNAME}
RUN echo "${UNAME}:${UNAME}" | chpasswd
RUN echo "${UNAME} ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
RUN chown -R ${UNAME}:${UNAME} /home/${UNAME}
RUN usermod -aG sudo ${UNAME}
RUN chsh -s /bin/bash ${UNAME}
ENV NOTVISIBLE "in users profile"
RUN echo "export VISIBLE=now" >> /etc/profile
RUN apt-get autoclean

USER ${UNAME}
WORKDIR /mnt
ENV LANG ja_JP.UTF-8
ENV SHELL /bin/bash
ENV HOME /home/${UNAME}

RUN echo "PS1='\[\e[37;45m\] \u \[\e[35;47m\]\[\e[30;47m\] \W \[\e[37;46m\]\[\e[30m\] $(__git_ps1 "(%s)") \[\e[36;49m\]\[\e[0m\]\n $ '" >> $HOME/.bashrc
