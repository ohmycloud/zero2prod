-- Add migration script here
-- We wrap the whole migration in a transaction to make sure
-- it succeeds or fails atomically.
-- `sqlx` does not do it automatically for us.
BEGIN;

-- BAckfill `status` for historical entries
UPDATE subscriptions
SET
    status = 'confirmed'
WHERE
    status IS NULL;

-- Make `status` mandatory
ALTER TABLE subscriptions
ALTER COLUMN status
SET
    NOT NULL;

COMMIT;
