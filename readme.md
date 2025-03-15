# SQL Migrate

```bash
psql -h localhost -p 5432 -U postgres
create database newsletter;
create database subscriptions;
sqlx migrate run --database-url=postgres://postgres:password@127.0.0.1:5432/newsletter

# or use this script
SKIP_DOCKER=true ./scripts/init_db.sh
```
