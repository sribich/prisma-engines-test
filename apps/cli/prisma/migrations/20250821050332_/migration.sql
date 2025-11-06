/*
  Warnings:

  - You are about to drop the `setting` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropIndex
DROP INDEX "setting_name_unique";

-- DropTable
PRAGMA foreign_keys=off;
DROP TABLE "setting";
PRAGMA foreign_keys=on;

-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Library" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "title" TEXT NOT NULL,
    "image_resource_id" BLOB,
    CONSTRAINT "Library_image_resource_id_fkey" FOREIGN KEY ("image_resource_id") REFERENCES "Resource" ("id") ON DELETE SET NULL ON UPDATE CASCADE
);
INSERT INTO "new_Library" ("id", "title") SELECT "id", "title" FROM "Library";
DROP TABLE "Library";
ALTER TABLE "new_Library" RENAME TO "Library";
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
