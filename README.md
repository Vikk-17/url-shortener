# Shorty — Ultra-Fast URL Shortener & Analytics

A high-performance, full-stack URL shortener built with **Rust (Actix-Web)**, **Redis Caching**, **PostgreSQL / Neon DB**, and a modern **React + Vite** frontend.

![Architecture Diagram](./shortener-backend/assets/images/shortner_fc.excalidraw.png)

---

## Key Features

* **Sub-Millisecond Redirection**: Caches active short links in Redis with 24-hour TTL for ultra-fast `302` redirects.
* **Collision-Resistant Base62 Encoding**: Unique deterministic slug generation based on PostgreSQL primary key sequences.
* **Neon DB Ready**: Fully compatible with Neon Serverless Postgres (`sslmode=require`).
* **Interactive React Frontend**: Glassmorphic dark mode UI built with Vite, TypeScript, live API health checks, and QR code generation.
* **Prometheus Observability**: Exposes real-time HTTP metrics on `/metrics` for Grafana dashboards.
* **Interactive API Documentation**: Built-in Swagger UI powered by Utoipa at `/swagger-ui/`.

---

## Technology Stack

| Tier | Technologies |
|---|---|
| **Backend** | Rust 2024, Actix-Web, SQLx, Deadpool Redis, Utoipa (OpenAPI), Prometheus |
| **Frontend** | React 19, Vite, TypeScript, Lucide Icons, QRCode, Glassmorphic CSS |
| **Database** | PostgreSQL / Neon Serverless Postgres |
| **Cache** | Redis 7 |
| **Containers** | Docker & Docker Compose |

---

## Repository Structure

```text
url-shortener/
├── compose.yaml                 # Master Docker Compose configuration
├── .env                         # Environment variables (DB & Redis credentials)
├── shortener-backend/           # Rust Actix-Web Backend
│   ├── src/
│   │   ├── main.rs              # Application entry point
│   │   ├── lib.rs               # Server setup & OpenAPI config
│   │   ├── handlers.rs          # HTTP route handlers (/shorten, /{slug})
│   │   ├── models.rs            # Structs & URL validation logic
│   │   ├── state.rs             # AppState (DB & Redis connection pools)
│   │   └── metrics.rs           # Prometheus counters
│   ├── migrations/              # SQLx database migrations
│   ├── prometheus/              # Prometheus scrape configuration
│   └── Dockerfile               # Multi-stage Rust build container
└── shortener-frontend/          # React + Vite Frontend
    ├── src/
    │   ├── App.tsx              # Main UI & state management
    │   ├── index.css            # Glassmorphic design tokens
    │   └── main.tsx             # React DOM entry
    └── Dockerfile               # Multi-stage Nginx production container
```

---

## Quick Setup Guide

### Prerequisites
* [Docker](https://docs.docker.com/get-docker/) & [Docker Compose](https://docs.docker.com/compose/install/)
* *(Optional for local dev)*: Rust (1.80+), Node.js (v20+), PostgreSQL, Redis

---

### 1. Environment Configuration

Create a `.env` file in the project root (`url-shortener/.env`):

```ini
# Database Connection String (Neon DB or local PostgreSQL)
DATABASE_URL="postgresql://user:password@ep-sample-123456.us-east-2.aws.neon.tech/neondb?sslmode=require"

# Redis Connection URL (for Docker Compose use redis://redis:6379)
REDIS_URL="redis://redis:6379"

# Application Base URL
BASE_URL="http://localhost:8080"

# Log Level
RUST_LOG="info"
```

---

### 2. Start with Docker Compose (Recommended)

Run the following command from the root directory:

```bash
docker compose up --build -d
```

This will build and start all 4 services:
* **Frontend Web App**: [http://localhost:3000](http://localhost:3000)
* **Backend API**: [http://localhost:8080](http://localhost:8080)
* **Swagger API Docs**: [http://localhost:8080/swagger-ui/](http://localhost:8080/swagger-ui/)
* **Prometheus Metrics**: [http://localhost:9090](http://localhost:9090)

To view live backend logs:
```bash
docker logs -f shorty_backend
```

To stop the stack:
```bash
docker compose down
```

---

## Manual Local Development Setup

If you prefer running the backend and frontend directly on your local machine:

### Backend Setup

1. Ensure Redis is running locally on port `6379`:
   ```bash
   docker run -d -p 6379:6379 --name shorty_redis redis:7-alpine
   ```

2. Update `shortener-backend/.env`:
   ```ini
   DATABASE_URL="postgresql://user:password@your-neon-host/neondb?sslmode=require"
   REDIS_URL="redis://127.0.0.1:6379"
   ```

3. Run database migrations:
   ```bash
   cd shortener-backend
   cargo install sqlx-cli
   sqlx migrate run
   ```

4. Start the backend:
   ```bash
   cargo run
   ```

---

### Frontend Setup

1. Navigate to the frontend directory:
   ```bash
   cd shortener-frontend
   ```

2. Install dependencies and start Vite dev server:
   ```bash
   npm install
   npm run dev
   ```
   Access the app at [http://localhost:5173](http://localhost:5173).

---

## Production Deployment Guide

### Option 1: VPS / Virtual Machine Deployment (Docker Compose)

Deploy to any Cloud Provider (AWS EC2, DigitalOcean, Hetzner, Linode, Vultr):

1. **Provision Virtual Server**: SSH into your server and install Docker & Docker Compose:
   ```bash
   curl -fsSL https://get.docker.com | sh
   ```
2. **Clone Repository & Configure Environment**:
   ```bash
   git clone <your-repo-url>
   cd url-shortener
   cp shortener-backend/.env.example .env
   # Edit .env with production Neon DB connection string and domain settings
   ```
3. **Launch Stack**:
   ```bash
   docker compose up -d --build
   ```
4. **Configure Nginx & SSL (Certbot)**:
   Point your domain (e.g. `shorty.yourdomain.com`) to your server IP using Nginx as a reverse proxy with Let's Encrypt TLS:
   ```nginx
   server {
       server_name shorty.yourdomain.com;
       location / {
           proxy_pass http://127.0.0.1:3000;
       }
       location /api/ {
           proxy_pass http://127.0.0.1:8080;
       }
   }
   ```
   Issue free SSL cert:
   ```bash
   sudo certbot --nginx -d shorty.yourdomain.com
   ```

---

### Option 2: Cloud Managed Deployment (Render / Railway + Vercel / Netlify)

#### Backend Deployment (Render / Railway / Fly.io)
1. **Database**: Create a PostgreSQL instance on [Neon.tech](https://neon.tech) and copy your connection string.
2. **Redis**: Create a free managed Redis database on [Upstash](https://upstash.com) and copy the TLS connection URL (`rediss://...`).
3. **Deploy Backend**:
   * Connect your GitHub repo to [Render](https://render.com) or [Fly.io](https://fly.io).
   * Select `shortener-backend` directory and choose **Docker** runtime.
   * Add environment variables:
     * `DATABASE_URL`: Your Neon connection string.
     * `REDIS_URL`: Your Upstash Redis connection string.
     * `BASE_URL`: Your deployed backend public URL (e.g., `https://shorty-api.onrender.com`).
     * `RUST_LOG`: `info`

#### Frontend Deployment (Vercel / Netlify / Cloudflare Pages)
1. Import `shortener-frontend` into [Vercel](https://vercel.com) or [Netlify](https://netlify.com).
2. Set Build Command: `npm run build` & Output Directory: `dist`.
3. Add Environment Variable:
   * `VITE_API_URL`: Set to your deployed backend URL (e.g. `https://shorty-api.onrender.com`).
4. Deploy! Your frontend is live with SSL out-of-the-box.

---

### Option 3: AWS CloudFront + S3 + Route 53 Custom Domain Deployment

This option hosts the frontend static files on an AWS S3 bucket behind AWS CloudFront CDN with a custom domain and free SSL certificate.

#### 1. Build Static Frontend Assets
Set your production backend URL in `shortener-frontend/.env`:
```ini
VITE_API_URL="https://api.yourdomain.com"
```
Build static assets:
```bash
cd shortener-frontend
npm run build
```
The static bundle will be generated in `shortener-frontend/dist/`.

#### 2. Create S3 Bucket
1. Go to AWS S3 Console -> **Create Bucket**.
2. Name: `shorty-frontend-bucket` (choose your preferred AWS region).
3. Keep **Block all public access** enabled (CloudFront will access S3 via Origin Access Control).
4. Upload all files from `shortener-frontend/dist/` into the S3 bucket root.

#### 3. Request SSL Certificate (AWS Certificate Manager)
1. Go to AWS Certificate Manager (ACM) in region **us-east-1** (CloudFront requires ACM certificates in `us-east-1`).
2. Click **Request Certificate** -> Public Certificate.
3. Add domain name: `shorty.yourdomain.com` (or `*.yourdomain.com`).
4. Choose **DNS Validation**. Add the generated CNAME record to your DNS provider (Route 53 or external DNS) to validate ownership.

#### 4. Create AWS CloudFront Distribution
1. Go to AWS CloudFront Console -> **Create Distribution**.
2. **Origin Domain**: Select your S3 bucket (`shorty-frontend-bucket.s3.amazonaws.com`).
3. **Origin Access**: Choose **Origin Access Control (OAC)** and attach the generated S3 bucket policy.
4. **Viewer Protocol Policy**: Select **Redirect HTTP to HTTPS**.
5. **Alternate Domain Names (CNAMEs)**: Enter `shorty.yourdomain.com`.
6. **Custom SSL Certificate**: Select the ACM certificate created in Step 3.
7. **Default Root Object**: Set to `index.html`.
8. **Custom Error Responses**: Add custom error page for SPA routing:
   * HTTP error code: `404`
   * Customize error response: `Yes`
   * Response page path: `/index.html`
   * HTTP Response code: `200`
9. Click **Create Distribution**. CloudFront will issue a distribution domain name (e.g. `d111111abcdef8.cloudfront.net`).

#### 5. Map Custom Domain in DNS (Route 53 or External DNS)
* **Using AWS Route 53**:
  1. Go to Route 53 Hosted Zone for `yourdomain.com`.
  2. Create Record -> Name: `shorty` (type `A` - IPv4).
  3. Enable **Alias** -> Route traffic to **Alias to CloudFront distribution**.
  4. Select your CloudFront distribution (`d111111abcdef8.cloudfront.net`).
* **Using External DNS (Namecheap, GoDaddy, Cloudflare, etc.)**:
  1. Add a `CNAME` record: Host `shorty` -> Value `d111111abcdef8.cloudfront.net`.

Once DNS propagates, your frontend application will be securely served worldwide over SSL at `https://shorty.yourdomain.com`.

---

## API Reference

| Method | Endpoint | Description | Request Body / Params |
|---|---|---|---|
| `POST` | `/api/v1/data/shorten` | Shorten a long URL | `{"longurl": "https://example.com"}` |
| `GET` | `/api/v1/{slug}` | HTTP 302 Redirect to long URL | Path param: `slug` |
| `GET` | `/metrics` | Prometheus metrics scrape endpoint | - |
| `GET` | `/swagger-ui/` | Interactive Swagger API documentation | - |

---

## Running Tests

To execute unit tests for URL validation and backend handlers:

```bash
cd shortener-backend
cargo test
```
