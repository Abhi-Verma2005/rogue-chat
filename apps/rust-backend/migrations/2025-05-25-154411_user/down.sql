-- This file should undo anything in `up.sql`
CREATE TABLE "_prisma_migrations"(
	"id" VARCHAR(36) NOT NULL PRIMARY KEY,
	"checksum" VARCHAR(64) NOT NULL,
	"finished_at" TIMESTAMPTZ,
	"migration_name" VARCHAR(255) NOT NULL,
	"logs" TEXT,
	"rolled_back_at" TIMESTAMPTZ,
	"started_at" TIMESTAMPTZ NOT NULL,
	"applied_steps_count" INT4 NOT NULL
);

DROP TABLE IF EXISTS "user";
