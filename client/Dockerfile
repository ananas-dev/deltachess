FROM node as builder
WORKDIR /deltachess-client
COPY package.json .
COPY yarn.lock .
RUN yarn install
COPY . .
RUN yarn build

FROM nginx as runtime
WORKDIR /deltachess-client
COPY --from=builder /deltachess-client/public .
COPY nginx.conf /etc/nginx/nginx.conf
