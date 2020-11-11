FROM gitpod/workspace-full-vnc

RUN apt-get update && apt-get install --no-install-recommends libasound2-dev libudev-dev