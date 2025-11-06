-- CreateTable
CREATE TABLE "Progress" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "timestamp" BIGINT NOT NULL,
    "media_id" BLOB NOT NULL,
    CONSTRAINT "Progress_media_id_fkey" FOREIGN KEY ("media_id") REFERENCES "Media" ("id") ON DELETE RESTRICT ON UPDATE RESTRICT
);

-- CreateIndex
CREATE UNIQUE INDEX "Progress_media_id_key" ON "Progress"("media_id");
