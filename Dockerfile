FROM python:3.11-slim as backend

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy requirements file
COPY blockchain_core/critter-craft/requirements.txt .

# Install Python dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Copy backend code
COPY blockchain_core /app/blockchain_core
COPY main.py Config.py pet_core.py /app/

# Set environment variables
ENV PYTHONUNBUFFERED=1
ENV PYTHONDONTWRITEBYTECODE=1
ENV FLASK_APP=main.py
ENV FLASK_ENV=production

# Expose backend port
EXPOSE 5000

# Command to run the application
CMD ["python", "main.py"]

# Frontend build stage
FROM node:18-alpine as frontend-build

WORKDIR /app

# Copy frontend package files
COPY frontend/package.json frontend/package-lock.json* frontend/yarn.lock* ./

# Install dependencies
RUN npm install || yarn install

# Copy frontend source code
COPY frontend/ ./

# Build frontend
RUN npm run build || yarn build

# Production stage
FROM nginx:alpine as production

# Copy built frontend assets from frontend-build stage
COPY --from=frontend-build /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Expose web port
EXPOSE 80

# Start nginx
CMD ["nginx", "-g", "daemon off;"]