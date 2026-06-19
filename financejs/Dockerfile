# syntax=docker/dockerfile:1

# Development image: runs the Vite dev server with hot-module reload.
# In docker-compose the project source is bind-mounted over /app and an anonymous
# volume preserves this Linux node_modules (so the host's Windows node_modules never
# leaks in). esbuild/rolldown native binaries are therefore the correct platform.
FROM node:20-bookworm-slim
WORKDIR /app

# Install dependencies first for better layer caching.
COPY package.json package-lock.json ./
RUN npm ci

# Copy the rest so the image is runnable on its own; in compose this is shadowed
# by the bind mount, which is what enables hot reload.
COPY . .

EXPOSE 3000
CMD ["npm", "run", "dev"]
