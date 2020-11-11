FROM gitpod/workspace-full-vnc

USER root

RUN apt-get clean
RUN apt-get update
RUN apt-get install -yq libasound2-dev libudev-dev
