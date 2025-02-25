# Markdown Exporter for LMStudio

It's a (dirty) program I wrote to export LMStudio conversations to Markdown.

If I gave it more time I would make a strongly typed data structure for conversations, for now it just reads JSON fields on the fly and prints its output to standard output.

## Usage & Tips

To run the program:

`cargo run -- [FILES] ...`

LMStudio conversations are located in `~/.cache/lm-studio/conversations` on UNIX systems, filenames are not very descriptive.

The easiest way to locate your conversation is to use "Show in File Explorer" from LMStudio.

To sort conversations by date : `find ~/.cache/lm-studio/conversations -printf "%T+ %p\n" | sort`

## Example

You'll find an example conversation I exported [there](./example.md)
