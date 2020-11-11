FROM gitpod/workspace-full-vnc

RUN apt-get clean && apt-get update
RUN apt-get install --no-install-recommends libasound2-dev libudev-dev
