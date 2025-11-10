# URL Shortner

### Back of the Envelope Estimation

Let's assume,
- write operation: 100 millions urls are generated per day.
- write operation per second: `100 millions / (24 * 3600) = 1158 (approx) ~ 1160`
- Read operation: Read to write operation `10:1`
- Read operation per second: `11600`
- The service will run for 10 years: `100 millions * 365 * 10 = 365 billions (approx)` record must support
- Total storage require: `365 billions * 100 bytes = 365 TB` if each url is `100 bytes` long

---

### Architecture

![DB Support](./assets/shortner_fc.excalidraw.png)

---

### TODO

- **Add Redis caching**
    - [x] Cache slug -> longurl
    - [x] Cache expiry (e.g., 24 hours)

- **Add stats endpoint**
    - [ ] GET /api/v1/stats/{slug}
    - [ ] Return:
        - created_at
        - click count
        - last_accessed
        - longurl

- **Redis (For rate limiting)**
    - [ ] Prevent spam on /shorten endpoint.
    - [ ] Or write basic Middleware for rate limiting
    - [ ] Check for aws options too for DDOS and rate limiting

- **Add logging or click tracking**
    - For click tracking use 302 redirecting

- **Add S3 log export endpoint in case of AWS integration**

- [x] CI/CD Pipelining
    - [x] Need to add docker secrets in the github

- **Dockerfile & docker-compose**
    - [x] Dockerfile - tested
    - [ ] API container
    - [ ] Postgres
    - [ ] Redis
    - [ ] LocalStack (S3) if needed

- **Production**
    - [ ] Middleware for tracing / logging
    - [ ] Clean error handling
    - [ ] URL validation + normalization
    - [ ] CORS setup

- **Think of using Monitoring tools like Prometheus & Grafana**

---

### References

- [Redis Cheatsheet](https://redis.io/learn/howtos/quick-start/cheat-sheet)
- [SCAN - Redis](https://redis.io/docs/latest/commands/scan/)
- [Redis 101 / GitHub](https://github.com/abhirockzz/rust-redis-101)
- [actix_session / Redis Client](https://github.com/actix/examples/blob/main/auth/redis-session/src/main.rs)
- [Deadpool Redis](https://docs.rs/deadpool-redis/latest/deadpool_redis/)
- [Lazy Static](https://docs.rs/lazy_static/latest/lazy_static/)
- [Prometheus Rust](https://docs.rs/prometheus/latest/prometheus/)
- [Prometheus / examples](https://github.com/tikv/rust-prometheus/tree/master/examples)
- [Actix web - Prometheus](https://docs.rs/actix-web-prometheus/latest/actix_web_prometheus/)
- [Prometheus Yaml Config](https://prometheus.io/docs/prometheus/latest/configuration/configuration/)
