/*
  Warnings:

  - You are about to drop the column `last_review` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `reading_audio` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `sentence_audio` on the `Card` table. All the data in the column will be lost.
  - Made the column `reading` on table `Card` required. This step will fail if there are existing NULL values in that column.
  - Made the column `sentence` on table `Card` required. This step will fail if there are existing NULL values in that column.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Card" (
    "id" BLOB NOT NULL PRIMARY KEY,
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
INSERT INTO "new_Card" ("difficulty", "due", "id", "reading", "sentence", "stability", "word") SELECT "difficulty", "due", "id", "reading", "sentence", "stability", "word" FROM "Card";
DROP TABLE "Card";
ALTER TABLE "new_Card" RENAME TO "Card";
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
