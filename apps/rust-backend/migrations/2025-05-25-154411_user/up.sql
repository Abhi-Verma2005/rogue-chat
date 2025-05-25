-- Your SQL goes here
DROP TABLE IF EXISTS "_prisma_migrations";
CREATE TABLE "user"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"username" VARCHAR NOT NULL,
	"password" VARCHAR NOT NULL,
	"email" VARCHAR NOT NULL
);

