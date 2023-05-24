
FROM ubuntu:latest


RUN apt-get update  -y && \
    apt-get upgrade  -y && \
    apt-get install curl -y &&\
    apt-get install cargo -y &&\
    mkdir /usr/local/topological-sort

 

VOLUME /topological-sort


WORKDIR /topological-sort
CMD ["topological-sort"]