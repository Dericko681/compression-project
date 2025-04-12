# Stage 1: Build stage
FROM node:16-alpine as builder

WORKDIR /usr/src/app

COPY package*.json ./
RUN npm ci --only=production

COPY . .

# Stage 2: Use Google Distroless Node.js base image
FROM gcr.io/distroless/nodejs:16

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/node_modules ./node_modules
COPY --from=builder /usr/src/app/lz.js ./lz.js

CMD ["jest"]
