-- CreateTable
CREATE TABLE "data_migration" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "name" TEXT NOT NULL,
    "executed_at" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- CreateTable
CREATE TABLE "Series" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "title" TEXT NOT NULL
);

-- CreateTable
CREATE TABLE "Media" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "title" TEXT NOT NULL,
    "kind" TEXT NOT NULL,
    "series_id" BLOB,
    "volume" INTEGER
);

-- CreateTable
CREATE TABLE "Book" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "path" TEXT NOT NULL,
    "rendered_path" TEXT NOT NULL,
    "audio_path" TEXT,
    "rendered_audio_path" TEXT,
    "media_id" BLOB NOT NULL,
    CONSTRAINT "Book_media_id_fkey" FOREIGN KEY ("media_id") REFERENCES "Media" ("id") ON DELETE RESTRICT ON UPDATE RESTRICT
);

-- CreateTable
CREATE TABLE "TimestampedBook" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "audio_path" TEXT NOT NULL,
    "parsed_audio_path" TEXT,
    "parsed_book_path" TEXT,
    "final_book_path" TEXT
);

-- CreateTable
CREATE TABLE "Card" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "cid" BLOB NOT NULL,
    "due" INTEGER NOT NULL,
    "word" TEXT NOT NULL,
    "reading" TEXT,
    "reading_path" TEXT,
    "sentence" TEXT,
    "sentence_path" TEXT,
    "definition_native" TEXT,
    "definition_tl" TEXT,
    "stability" REAL,
    "difficulty" REAL
);

-- CreateTable
CREATE TABLE "Dictionary" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "title" TEXT NOT NULL,
    "language_type" TEXT NOT NULL,
    "kinds" TEXT NOT NULL,
    "file_path" TEXT NOT NULL,
    "data_path" TEXT NOT NULL,
    "rank" TEXT NOT NULL
);

-- CreateTable
CREATE TABLE "Word" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "word" TEXT NOT NULL,
    "reading" TEXT NOT NULL,
    "definition" TEXT NOT NULL,
    "dictionary_id" BLOB NOT NULL,
    CONSTRAINT "Word_dictionary_id_fkey" FOREIGN KEY ("dictionary_id") REFERENCES "Dictionary" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "Frequency" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "word" TEXT NOT NULL,
    "reading" TEXT NOT NULL,
    "frequency" INTEGER NOT NULL,
    "dictionary_id" BLOB NOT NULL,
    CONSTRAINT "Frequency_dictionary_id_fkey" FOREIGN KEY ("dictionary_id") REFERENCES "Dictionary" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "PitchAccent" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "word" TEXT NOT NULL,
    "reading" TEXT NOT NULL,
    "position" INTEGER NOT NULL,
    "dictionary_id" BLOB NOT NULL,
    CONSTRAINT "PitchAccent_dictionary_id_fkey" FOREIGN KEY ("dictionary_id") REFERENCES "Dictionary" ("id") ON DELETE CASCADE ON UPDATE CASCADE
);

-- CreateTable
CREATE TABLE "setting" (
    "id" BLOB NOT NULL PRIMARY KEY,
    "name" TEXT NOT NULL,
    "kind" TEXT NOT NULL,
    "value" TEXT NOT NULL,
    "constraints" TEXT
);

-- CreateIndex
CREATE UNIQUE INDEX "data_migration_name_uniq" ON "data_migration"("name");

-- CreateIndex
CREATE UNIQUE INDEX "library_title_uniq" ON "Media"("title");

-- CreateIndex
CREATE UNIQUE INDEX "Book_media_id_key" ON "Book"("media_id");

-- CreateIndex
CREATE INDEX "Dictionary_rank_idx" ON "Dictionary"("rank" ASC);

-- CreateIndex
CREATE INDEX "Frequency_word_idx" ON "Frequency"("word");

-- CreateIndex
CREATE UNIQUE INDEX "setting_name_unique" ON "setting"("name");
