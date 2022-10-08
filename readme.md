# RLE
this programme uses run length encoding to compress images and decompress images. It also comes with support for creating new image files using the pbm file format.

# How to use

## Quick Start

#### cargo run input.txt 1 output.txt output.pbm
converts the input.txt to an image
and compresses input.txt to output.txt

#### cargo run output.txt 0 input.txt output.pbm
converts the output.txt to an image
and decompresses output.txt to input.txt

#### cargo run inputfile mode outputfile image_output_file

## Arguments
### inputfile
#### default: pattern.txt
first argument to pass, should be a .txt file but this is not checked anywhere in the program so use other filetypes at your own risk

### mode
#### default: 1 (compression mode)
second argument to pass, should be either a 1 or a 0, this is not checked but the program will fail to run if the value is not one of these two

### outputfile
#### default: output.txt
third argument to pass, should be a .txt file but this is not checked anywhen in the program so use other filetypes at your own risk

### image output file
#### default: output.pbm
fourth argument to pass, should be a .pbm file as it will not work with any other file formats.

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

## Libraries used
### std::fs
used for reading from files and writing to files

### std::env
used for getting the arguments passed to cargo run