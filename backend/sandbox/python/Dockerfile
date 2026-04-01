FROM python:3.11-alpine

LABEL evolith.sandbox=true
LABEL evolith.runtime=python311

RUN apk add --no-cache --virtual .build-deps \
    build-base \
    libffi-dev \
    openblas-dev \
    freetype-dev \
    libpng-dev \
    && apk add --no-cache \
    libstdc++ \
    openblas \
    freetype \
    libpng

RUN addgroup -g 1000 -S sandbox && \
    adduser -u 1000 -S sandbox -G sandbox -h /workspace -s /bin/sh

WORKDIR /workspace

RUN pip install --no-cache-dir \
    numpy \
    pandas \
    scipy \
    matplotlib \
    requests \
    pyyaml

RUN apk del .build-deps && \
    rm -rf /root/.cache /tmp/*

ENV PYTHONDONTWRITEBYTECODE=1 \
    PYTHONUNBUFFERED=1

USER sandbox

CMD ["tail", "-f", "/dev/null"]
