-- CreateTable
CREATE TABLE "Resource" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "hash" TEXT NOT NULL,
    "path" TEXT NOT NULL,
    "mime_type" TEXT NOT NULL
);
