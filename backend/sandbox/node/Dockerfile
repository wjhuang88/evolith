FROM node:20-alpine

LABEL evolith.sandbox=true
LABEL evolith.runtime=node20

RUN addgroup -g 1000 -S sandbox && \
    adduser -u 1000 -S sandbox -G sandbox -h /workspace -s /bin/sh

WORKDIR /workspace

RUN npm install -g \
    typescript \
    tsx \
    lodash \
    axios \
    dayjs && \
    npm cache clean --force && \
    rm -rf /tmp/*

ENV NODE_ENV=production

USER sandbox

CMD ["tail", "-f", "/dev/null"]
