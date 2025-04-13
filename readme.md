# SQL Migrate

```bash
psql -h localhost -p 5432 -U postgres
create database newsletter;
create database subscriptions;
sqlx migrate run --database-url=postgres://postgres:password@127.0.0.1:5432/newsletter

# or use this script
SKIP_DOCKER=true ./scripts/init_db.sh
```

# Redis setup

```bash
. ./scripts/init_redis.sh
```

# How to use

```bash
curl http://127.0.0.1:8000/health_check

# Send an email with curl
curl "https://api.postmarkapp.com/email" \
  -X POST \
  -H "Accept: application/json" \
  -H "Content-Type: application/json" \
  -H "X-Postmark-Server-Token: POSTMARK_API_TEST" \
  -d '{"From": "cloud@ohmycloudy.uk", "To": "cloud@ohmycloudy.uk", "Subject": "Hello from Postmark", "HtmlBody": "<strong>Hello</strong> dear Postmark user."}'
```
