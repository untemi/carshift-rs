CREATE TABLE "users" (
	"id" INTEGER NOT NULL UNIQUE,
	"username" TEXT NOT NULL,
	"passhash" TEXT NOT NULL,
	"firstname" TEXT NOT NULL,
	"lastname" TEXT,
	"phone" TEXT,
	"email" TEXT,
	"pfp_file" TEXT,
	PRIMARY KEY("id")
);

CREATE UNIQUE INDEX "users_index_0"
ON "users" ("username");
