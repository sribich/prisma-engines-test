/*
  Warnings:

  - Added the required column `state` to the `Card` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Card" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "state" TEXT NOT NULL,
    "step" INTEGER,
    "due" BIGINT NOT NULL,
    "word" TEXT NOT NULL,
    "reading" TEXT NOT NULL,
    "reading_audio_id" BLOB,
    "sentence" TEXT NOT NULL,
    "sentence_audio_id" BLOB,
    "stability" REAL,
    "difficulty" REAL,
    CONSTRAINT "Card_reading_audio_id_fkey" FOREIGN KEY ("reading_audio_id") REFERENCES "Resource" ("id") ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT "Card_sentence_audio_id_fkey" FOREIGN KEY ("sentence_audio_id") REFERENCES "Resource" ("id") ON DELETE SET NULL ON UPDATE CASCADE
);
INSERT INTO "new_Card" ("difficulty", "due", "id", "reading", "reading_audio_id", "sentence", "sentence_audio_id", "stability", "word") SELECT "difficulty", "due", "id", "reading", "reading_audio_id", "sentence", "sentence_audio_id", "stability", "word" FROM "Card";
DROP TABLE "Card";
ALTER TABLE "new_Card" RENAME TO "Card";
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
