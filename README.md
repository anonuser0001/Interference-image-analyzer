# Interference image analyzer
Interference image analyzer is the program with the help of which the intensity distributions of the interference fringes are obtained. 
The program scans the image by passing through lines one pixel wide and draws a graph by calculating the intensity of the corresponding color at that pixel.

## Image processing 
In order for the program to successfully analyze the image, the following conditions must be met:
- The interference fringes should be horizontal. 
- The interference fringes should be red or green in color.
- It is desirable for the image to be cropped to contain those fringes that are clearly visible.

## Loading Images 
After launching the program, a window for selecting an interference image is opened.
- By clicking the Open file button, a window for selecting an image (in .png or .jpg format) from the file system opens.
- After selecting the image, its preview and the Analyze button appear.
- By clicking the Analyze button, a new window with intensity distributions of the given image opens. 

## Analysis of intensity distributions 
As an output from the program, you will obtain as many graphics as the width of the image in pixels.
- To display the corresponding intensity distribution, first, the color of the interference fringes should be selected by clicking the Red or Green button in the Color channel row.
- In the Line index row, by clicking the +/- buttons or dragging the slider, the intensity distribution of the selected vertical line of the input image can be viewed.
- By clicking the Copy coordinates to clipboard button, located in the top right corner of the Analyze window, the coordinates of the observed intensity distribution are copied for further analysis in the desired program.

## Averaging intensity distributions 
With the help of this program, averaging can be performed to more precisely determine the position of the maximum (center) of the interference lines.
- To add the observed intensity distribution to the list for averaging, Add to averaged graph button should be clicked.
- The list of added distributions is located on the right side of the window, with options to show (Show button) or remove (Remove button) distributions from the list.
- The averaged distribution can be visualized by clicking the Averaging button in the Mode row.
