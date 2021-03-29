FROM nginx:alpine

RUN rm -v /etc/nginx/nginx.conf

ADD nginx.conf /etc/nginx/

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]