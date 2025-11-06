/*
  Warnings:

  - You are about to drop the column `audio_resource` on the `Book` table. All the data in the column will be lost.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Book" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "path" TEXT NOT NULL,
    "rendered_path" TEXT NOT NULL,
    "rendered_audio_path" TEXT,
    "audio_resource_id" BLOB,
    "media_id" BLOB NOT NULL,
    CONSTRAINT "Book_audio_resource_id_fkey" FOREIGN KEY ("audio_resource_id") REFERENCES "Resource" ("id") ON DELETE SET NULL ON UPDATE CASCADE,
    CONSTRAINT "Book_media_id_fkey" FOREIGN KEY ("media_id") REFERENCES "Media" ("id") ON DELETE RESTRICT ON UPDATE RESTRICT
);
INSERT INTO "new_Book" ("id", "media_id", "path", "rendered_audio_path", "rendered_path") SELECT "id", "media_id", "path", "rendered_audio_path", "rendered_path" FROM "Book";
DROP TABLE "Book";
ALTER TABLE "new_Book" RENAME TO "Book";
CREATE UNIQUE INDEX "Book_media_id_key" ON "Book"("media_id");
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
