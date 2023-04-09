FROM node:18-alpine as build
WORKDIR /app
RUN npm i -g pnpm@8

COPY .npmrc .eslint* *.config.cjs svelte.config.js vite.config.ts tsconfig.json ./
COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY ./src ./src
ARG VITE_API_URL
RUN pnpm exec svelte-kit sync
RUN pnpm build

FROM nginx:stable-alpine

COPY --from=build /app/build /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
ENTRYPOINT [ "nginx", "-g", "daemon off;" ]
