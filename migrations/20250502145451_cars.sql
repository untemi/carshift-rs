CREATE TABLE "cars" (
	"id" INTEGER NOT NULL UNIQUE,
	"name" TEXT NOT NULL,
	"price" REAL NOT NULL,
	"start_at" DATE,
	"end_at" DATE,
	"owner" INTEGER NOT NULL,
	"district" INTEGER NOT NULL,
	PRIMARY KEY("id"),
	FOREIGN KEY ("owner") REFERENCES "users"("id")
	ON UPDATE NO ACTION ON DELETE CASCADE,
	FOREIGN KEY ("district") REFERENCES "districts"("id")
	ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE UNIQUE INDEX "cars_index_0"
ON "cars" ("name");
