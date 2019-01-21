FROM amd64/alpine:3.6

ADD bin/start.sh start.sh
RUN chmod +x start.sh
ADD build/linux linux
RUN chmod +x linux

EXPOSE 80

ENV PORT 80

CMD ["sh", "./start.sh", "/tmp", "/tmp/data/data.zip", "./linux"]
