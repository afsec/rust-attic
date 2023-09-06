# PIPELINE: development
# FROM alpine:latest
# ENV PIPELINE development

# PIPELINE: development
# Add `tcpdump bind-tools` and clean
# RUN apk add --update tcpdump bind-tools \
#&& rm  -rf /tmp/* /var/cache/apk/*

# PIPELINE: staging
# FROM busybox:musl
# ENV PIPELINE staging

# PIPELINE: release (scratch)
FROM scratch
ENV PIPELINE release

# PIPELINE: development
#COPY ./entrypoint.sh /

# PIPELINE: staging/release
COPY ./dist/quicknet_client /

# PIPELINE: development
#RUN chmod +x /entrypoint.sh

# PIPELINE: development
# ENTRYPOINT ["/entrypoint.sh"] 

# PIPELINE: staging/release
ENTRYPOINT ["/quicknet_client"]
