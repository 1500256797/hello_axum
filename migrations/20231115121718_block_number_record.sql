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

-- Records of BlockNumberRecord
INSERT INTO "block_number_record" ("id", "chain_type", "wss_url", "https_url", "contract_addr", "event_type", "event_signature", "from_block", "last_block", "desc", "sechdule_time", "space_block") VALUES
  (1, 'bsc', 'wss://bsc-mainnet.nodereal.io/ws/v1/3ddb7e8e535747c4a2209cdace70ff29', '', '0x91db77bBe3a79b654137f58157C41267A9830792', 'JoinActivity', 'event JoinActivity(address indexed user,uint256 indexed activityId,bool indexed isWinner)', 31244297, 31244297, '', 20, 4000),
  (6, 'bsc', 'wss://bsc-mainnet.nodereal.io/ws/v1/3ddb7e8e535747c4a2209cdace70ff29', '', '0x9677bE3ca5ee3F5973D91FF053F0BA1eCa59809F', 'GalleryChanged', 'event GalleryChanged(uint256,string,address,uint256,address,string,string,string,bool,uint64,address[],uint256[],address,address[])', 31156122, 31156122, '', 30, 4000),
  (7, 'bsc', 'wss://bsc-mainnet.nodereal.io/ws/v1/3ddb7e8e535747c4a2209cdace70ff29', '', '0x91db77bBe3a79b654137f58157C41267A9830792', 'WinnerErc20Amount', 'event WinnerErc20Amount(address indexed user,uint256 indexed activityId,uint256 indexed amount)', 33364491, 33364491, '', 20, 4000);
