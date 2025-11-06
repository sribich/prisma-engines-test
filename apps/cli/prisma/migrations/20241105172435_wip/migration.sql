/*
  Warnings:

  - The primary key for the `Card` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `cid` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `definition_native` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `definition_tl` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `reading_path` on the `Card` table. All the data in the column will be lost.
  - You are about to drop the column `sentence_path` on the `Card` table. All the data in the column will be lost.
  - You are about to alter the column `id` on the `Card` table. The data in that column could be lost. The data in that column will be cast from `Int` to `Binary`.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Card" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "due" INTEGER NOT NULL,
    "word" TEXT NOT NULL,
    "reading" TEXT,
    "reading_audio" TEXT,
    "sentence" TEXT,
    "sentence_audio" TEXT,
    "stability" REAL,
    "difficulty" REAL
);
INSERT INTO "new_Card" ("difficulty", "due", "id", "reading", "sentence", "stability", "word") SELECT "difficulty", "due", "id", "reading", "sentence", "stability", "word" FROM "Card";
DROP TABLE "Card";
ALTER TABLE "new_Card" RENAME TO "Card";
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
