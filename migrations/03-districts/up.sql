CREATE TABLE "districts" (
	"id" INTEGER NOT NULL UNIQUE,
	"name" TEXT NOT NULL,
	PRIMARY KEY("id")
);

CREATE UNIQUE INDEX "districts_index_0"
ON "districts" ("name");


INSERT INTO districts (name) 
  VALUES 
		("Aïn Chock"),
		("Aïn Sebaâ"),
		("Al Fida"),
		("Ben M'Sick"),
		("Hay Hassani"),
		("Moulay Rachid"),
		("Sidi Bernoussi");
