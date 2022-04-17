# Searcher
Searcher is a program for searching text files

You can run searcher by typing `search word1 word2 word3` into a terminal. Searcher will then print out the names of text documents. The first document will be the best match, the second will be the second best match,
etc.

<!-- By default searcher will return every document which contains at least one word you searched for. If you want fewer you can use the `--limit` or `-l` flag. You can see other flags in the [Flags](#flags) section of this file. -->

Searcher can search only for the words located in /usr/share/dict/words

When you run searcher it will look for a `.srch` file. If one exists it will use it to search that directory. Otherwise it will read through all text files and create a `.srch` file. If a `.words` file is present, it will use that file as its dictionary.

<!-- ## Flags

- `--limit` or `-n` Limit the number of words searcher will search for
- `--exact` or `-e` Match exactly these words in this order
- `--re-process` or `-rp` Re-process the documents in this folder
- `--local` or `-l` Make a local words file. Use when `/usr/share/dict/words` doesn't contain some common words in your files
- `--snipet` or `-s` Show the snipet in each file that matches these words
- `--snipet-lenth` or `-sl` Set the length of snipets -->

## The `.srch` file

The `.srch` file contains a bitmap where each row represents a file and each bit represents a word (the words provided in `/usr/share/dict/words` or a local `.words` file - see [the .words file](#the-words-file)). Prepending each row is the name of the file it refers to which is `null` terminated.

## The `.words` file

The `.words` file contains a new-line delimited list of each unique word in every file in the directory. The words are sorted alphabetically.

## The searching algorithm

When searching, your words are converted into a row matching the `.srch` format - as if your words were a file in the folder - which is then bit-wise anded with every row in the `.srch` file. Every non-empty row is then sorted by the number of matching words.

<!-- If the `--snipet` flag is present, then the program will convert the bits in each matching row to a list of matching words. Each matching file is then searched for each occurence of a matching word. These occurences are returned as `Match` objects containing the index of a match and the matching word. Matches are then processed into snipets, a snipet contains the words within the snipet length from a match.

If matches are close enough together (i.e. within the snipet length), they will be combined into one snipet. Snipets are scored based on 1) the number of matches they contain and 2) whether the matches are in order. Files are then scored on the combined score of their snippets and resorted. -->
