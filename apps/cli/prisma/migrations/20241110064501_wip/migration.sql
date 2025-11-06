-- CreateTable
CREATE TABLE "Pronunciation" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "word" TEXT NOT NULL,
    "reading" TEXT,
    "name" TEXT NOT NULL,
    "sex" TEXT NOT NULL,
    "language" TEXT NOT NULL
);

-- CreateIndex
CREATE INDEX "Pronunciation_word_idx" ON "Pronunciation"("word");
