/*
  Warnings:

  - Added the required column `path` to the `Pronunciation` table without a default value. This is not possible if the table is not empty.

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
    "path" TEXT NOT NULL
);
INSERT INTO "new_Pronunciation" ("id", "language", "name", "reading", "sex", "word") SELECT "id", "language", "name", "reading", "sex", "word" FROM "Pronunciation";
DROP TABLE "Pronunciation";
ALTER TABLE "new_Pronunciation" RENAME TO "Pronunciation";
CREATE INDEX "Pronunciation_word_idx" ON "Pronunciation"("word");
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
