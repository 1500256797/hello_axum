-- Add migration script here
-- Alter the table to change data type from citext to varchar
ALTER TABLE "block_number_record"
  ALTER COLUMN "chain_type" TYPE varchar(191),
  ALTER COLUMN "event_type" TYPE varchar(191);
