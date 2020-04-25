FROM busybox:musl
COPY ./dist/concierge-db /
EXPOSE 3341
ENTRYPOINT ["/concierge-db"]
