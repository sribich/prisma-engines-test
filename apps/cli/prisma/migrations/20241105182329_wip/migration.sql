/*
  Warnings:

  - Added the required column `last_review` to the `Card` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Card" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "due" BIGINT NOT NULL,
    "last_review" BIGINT NOT NULL,
    "word" TEXT NOT NULL,
    "reading" TEXT,
    "reading_audio" TEXT,
    "sentence" TEXT,
    "sentence_audio" TEXT,
    "stability" REAL,
    "difficulty" REAL
);
INSERT INTO "new_Card" ("difficulty", "due", "id", "reading", "reading_audio", "sentence", "sentence_audio", "stability", "word") SELECT "difficulty", "due", "id", "reading", "reading_audio", "sentence", "sentence_audio", "stability", "word" FROM "Card";
DROP TABLE "Card";
ALTER TABLE "new_Card" RENAME TO "Card";
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
