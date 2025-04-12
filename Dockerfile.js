# Stage 1: Build JS application
FROM --platform=$BUILDPLATFORM node:20-alpine AS builder

WORKDIR /app

# Cache dependencies
COPY js-compressor/package.json js-app/package-lock.json ./
RUN npm ci

# Copy source and build
COPY js-compressor/ ./
RUN npm run build

# Stage 2: Runtime image
FROM node:20-alpine

WORKDIR /app

# Copy built application
COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/package.json ./

# Non-root user for security
RUN addgroup -S appgroup && \
    adduser -S appuser -G appgroup && \
    chown -R appuser:appgroup /app
USER appuser

# Health check
HEALTHCHECK --interval=30s --timeout=3s \
  CMD curl -f http://localhost:3000/health || exit 1

EXPOSE 3000
CMD ["npm", "start"]