-- Create extension for case-insensitive citext type
CREATE EXTENSION IF NOT EXISTS citext;

-- Table structure for BlockNumberRecord
CREATE TABLE IF NOT EXISTS "block_number_record" (
  "id" serial PRIMARY KEY,
  "chain_type" citext NOT NULL DEFAULT 'bsc',
  "wss_url" varchar(191) NOT NULL DEFAULT '',
  "https_url" varchar(191) NOT NULL DEFAULT '',
  "contract_addr" varchar(191) NOT NULL DEFAULT '',
  "event_type" citext NOT NULL DEFAULT 'JoinActivity',
  "event_signature" varchar(191) NOT NULL DEFAULT '',
  "from_block" integer NOT NULL DEFAULT '31244297',
  "last_block" integer NOT NULL,
  "desc" varchar(191) NOT NULL DEFAULT '',
  "sechdule_time" integer NOT NULL DEFAULT '60',
  "space_block" integer NOT NULL DEFAULT '40000',
  CONSTRAINT "BlockNumberRecord_chain_type_event_type_key" UNIQUE ("chain_type", "event_type")
);
