/*
  Warnings:

  - You are about to drop the column `series_id` on the `Media` table. All the data in the column will be lost.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Media" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "title" TEXT NOT NULL,
    "kind" TEXT NOT NULL,
    "library_id" BLOB,
    "volume" INTEGER
);
INSERT INTO "new_Media" ("id", "kind", "title", "volume") SELECT "id", "kind", "title", "volume" FROM "Media";
DROP TABLE "Media";
ALTER TABLE "new_Media" RENAME TO "Media";
CREATE UNIQUE INDEX "library_title_uniq" ON "Media"("title");
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
