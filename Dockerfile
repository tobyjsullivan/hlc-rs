FROM alpine:3.6

ADD bin/start.sh start.sh
RUN chmod u+x start.sh
ADD build/linux linux
RUN chmod u+x linux

EXPOSE 80

ENV PORT 80

CMD ["./start.sh", "/tmp", "/tmp/data/data.zip", "./linux"]
