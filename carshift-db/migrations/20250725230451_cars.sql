CREATE TABLE "cars" (
	"id" INTEGER NOT NULL UNIQUE,
	"name" TEXT NOT NULL,
	"price" REAL NOT NULL,
	"start_date" DATE NOT NULL,
	"end_date" DATE NOT NULL,
	"owner" INTEGER NOT NULL,
	"district" INTEGER NOT NULL,
  "description" TEXT,
  "pic_file" TEXT NOT NULL,

	PRIMARY KEY("id"),
	FOREIGN KEY ("owner") REFERENCES "users"("id")
	ON UPDATE NO ACTION ON DELETE CASCADE,
	FOREIGN KEY ("district") REFERENCES "districts"("id")
	ON UPDATE NO ACTION ON DELETE CASCADE
);

CREATE INDEX "cars_index_0"
ON "cars" ("name");
