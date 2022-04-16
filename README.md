# Searcher
Searcher is a program for searching text files

You can run searcher by typing `search word1 word2 word3` into a terminal. Searcher will then print out the names of text documents. The first document will be the best match, the second will be the second best match,
etc.

By default searcher will return every document which contains at least one word you searched for. If you want fewer you can use the `--limit` or `-l` flag. You can see other flags in the [Flags](#flags) section of this file.

Searcher can search only for the words located in /usr/share/dict/words

When you run searcher it will look for a `.srch` file. If one exists it will use it to search that directory. Otherwise - if the `--re-process` flag is present - it will read through all text files and create a `.srch` file

## Flags

- `--limit / -l` Limit the number of words searcher will search for
- `--exact / -e` Match exactly these words in this order
- `--re-process / -rp` Re-process the documents in this folder
