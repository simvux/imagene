# Imagene - A General Purpose Image Manipulation Tool

## Example
![frog.png](example/frog.jpg)
```yaml
imagene frog.jpg resize:500,0 append:apple.jpg result.png
```
![result.png](example/result.png)

## Building
`cargo build --release`

## Usage
`imagene --help`
```shell
Syntax:
    imagene <infile> ...<flag>... ...<action>:<value>... <outfile>

Available Actions:
    brightness:<int>           -> Increase brightness by percent
    contrast:<int>             -> Increase contrast by percent
    blur:<float>               -> Add gaussian blur by sigma (recommended 1-20)
    unsharpen:<float,int>      -> Add unsharpen mask with float being sigma and int being threshold
    flip:<v/h>                 -> Flip image v for vertically or h for horizontally
    rotate:<left/right/down>   -> Rotate an image by 90,180,270 degrees
    resize:<int,int>           -> Resize an image, leave one of the ints empty to auto scale it
    crop:<int,int,int,int>     -> Crop an image (x,y,width,height)
    append:<string,left/under> -> Add another image next to source image
    format:<string>            -> Specify output image format
    format:<jpg,int>           -> For JPG, also specify quality

Available Flags:

Examples:
     -> Increases the contrast of the original image by 20% and adds an extra image next to it
     imagene in_file.png contrast:20 append:extra_image.png out_file.png

     -> Set width to 2000, automatically scales height to keep aspect ratio and output to STDOUT
     imagene in_file.png resize:2000,0 stdout

     -> Overwrites an image with increased contrast
     imagene in_file.png contrast:2 in_file.png

```
