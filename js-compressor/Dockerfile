# Stage 1: Build stage
FROM node:16-alpine as builder

WORKDIR /usr/src/app

COPY package*.json ./
RUN npm ci --only=production

COPY . .

# Stage 2: Production stage
FROM node:16-alpine

WORKDIR /usr/src/app

# Create a non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Copy application files
COPY --from=builder --chown=appuser:appgroup /usr/src/app/node_modules ./node_modules
COPY --from=builder --chown=appuser:appgroup /usr/src/app/package.json ./package.json
COPY --from=builder --chown=appuser:appgroup /usr/src/app/cli.js ./cli.js
COPY --from=builder --chown=appuser:appgroup /usr/src/app/le.js ./le.js
COPY --from=builder --chown=appuser:appgroup /usr/src/app/lz.js ./lz.js

# Make the CLI executable
RUN chmod +x ./cli.js

# Switch to non-root user
USER appuser

# Set the entrypoint to our CLI
ENTRYPOINT ["node", "cli.js"]

EXPOSE 8000

CMD ["node", "cli.js"]
