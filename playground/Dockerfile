FROM ubuntu:latest
RUN apt-get update && apt-get install -y \
  supervisor \
  vim \
  curl
RUN mkdir -p /var/log/supervisor
COPY supervisord.conf /etc/supervisor/conf.d/supervisord.conf
COPY cwd.sh /cwd.sh

CMD ["/usr/bin/supervisord"]
# CMD ["tail", "-f", "/dev/null"]
