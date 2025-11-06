/*
  Warnings:

  - You are about to drop the column `path` on the `Pronunciation` table. All the data in the column will be lost.
  - Added the required column `resource_id` to the `Pronunciation` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Pronunciation" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "word" TEXT NOT NULL,
    "reading" TEXT,
    "name" TEXT NOT NULL,
    "sex" TEXT NOT NULL,
    "language" TEXT NOT NULL,
    "resource_id" BLOB NOT NULL,
    CONSTRAINT "Pronunciation_resource_id_fkey" FOREIGN KEY ("resource_id") REFERENCES "Resource" ("id") ON DELETE RESTRICT ON UPDATE CASCADE
);
INSERT INTO "new_Pronunciation" ("id", "language", "name", "reading", "sex", "word") SELECT "id", "language", "name", "reading", "sex", "word" FROM "Pronunciation";
DROP TABLE "Pronunciation";
ALTER TABLE "new_Pronunciation" RENAME TO "Pronunciation";
CREATE UNIQUE INDEX "Pronunciation_resource_id_key" ON "Pronunciation"("resource_id");
CREATE INDEX "Pronunciation_word_idx" ON "Pronunciation"("word");
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
