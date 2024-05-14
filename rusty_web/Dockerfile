FROM node:22 as build

WORKDIR /app

COPY package.json ./
COPY package-lock.json ./
RUN npm install
COPY . ./
RUN npm run build

FROM node:22-alpine3.19

ENV NODE_ENV=production

WORKDIR /app
COPY --from=build /app .

ENV HOST=0.0.0.0
EXPOSE 8080
CMD ["npm","run", "preview","--", "--host", "0.0.0.0", "--port", "8080"]
