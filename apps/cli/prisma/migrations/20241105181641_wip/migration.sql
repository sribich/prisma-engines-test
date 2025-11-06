/*
  Warnings:

  - You are about to alter the column `due` on the `Card` table. The data in that column could be lost. The data in that column will be cast from `Int` to `BigInt`.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Card" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "due" BIGINT NOT NULL,
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
