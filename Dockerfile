FROM busybox:musl
COPY ./dist/concierge-db /
COPY ./dist/database.sqlite3 /
EXPOSE 3341
ENTRYPOINT ["/concierge-db"]
