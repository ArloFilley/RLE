# RLE
this programme uses run length encoding to compress images and decompress images. It also comes with support for creating new image files using the pbm file format.

# How to use

## Clap Help Message
```
Usage: rle [OPTIONS] --input-file <INPUT_FILE> --output-file <OUTPUT_FILE>

Options:
  -i, --input-file <INPUT_FILE>    
  -o, --output-file <OUTPUT_FILE>  
  -d, --decode                     
  -e, --encode                     
  -s, --save-image                 
  -w, --width <WIDTH>              
  -h, --help                       Print help
  -V, --version                    Print version
```

# how it works
## RLE
This program uses run length encoding (RLE), this is a form of lossless compression. It takes a string of 1's and 0's from a text file and uses the rust file system to compress these into a shortened sequence.

It compresses these into a series of numbers with commas inbetween. This program assumes that the values start at 1 before flipping.

This program can also decompress the files it creates back into the originial input this can be checked by running these commands in the console.

#### diff inputfile outputfile
#### diff pattern.txt output.txt
if the program has run correctly there should be no output to the console.

## Creating Images
for creating images the .pbm file format. For this file format there is a header that contains a comment the type (ascii or binary), the dimensions of the image. The content uses an string of 1's represeting black and 0's represeting white seperated by a space
