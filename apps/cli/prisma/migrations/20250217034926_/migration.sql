/*
  Warnings:

  - You are about to drop the `Series` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropTable
PRAGMA foreign_keys=off;
DROP TABLE "Series";
PRAGMA foreign_keys=on;

-- CreateTable
CREATE TABLE "Library" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "title" TEXT NOT NULL
);
