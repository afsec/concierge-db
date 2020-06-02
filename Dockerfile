# Staging image (busybox:musl)
# FROM busybox:musl

# Production image (scratch)
FROM scratch
ENV PIPELINE development
COPY ./dist/concierge-db /
COPY ./dist/database.sqlite3 /
EXPOSE 3341
ENTRYPOINT ["/concierge-db"]
