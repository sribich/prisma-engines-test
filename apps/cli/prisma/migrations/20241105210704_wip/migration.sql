-- CreateTable
CREATE TABLE "CardReview" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "date" BIGINT NOT NULL,
    "rating" INTEGER NOT NULL,
    "card_id" BLOB NOT NULL,
    CONSTRAINT "CardReview_card_id_fkey" FOREIGN KEY ("card_id") REFERENCES "Card" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);
