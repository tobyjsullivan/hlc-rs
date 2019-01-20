FROM alpine:3.6

ADD bin/start.sh start.sh
ADD build/linux linux

EXPOSE 80

ENV PORT 80

CMD ["./start.sh", "/tmp", "/tmp/data/data.zip"]
