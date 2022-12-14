# SSD1306 OLED Emulator

An open-source emulator for small OLED displays that are often used for Arduino/IOT projects.

This project is still in an alpha build so it may be a little unstable.

![](https://i.imgur.com/TmFJ7hC.gif)

# Table of contents
1. [Who is this for?](#who-is-this-for)
2. [How to use](#how-to-use)
3. [Compiling from source](#compiling-from-source)
4. [Coming soon](#coming-soon)

## Who is this for?
Anyone wanting to see how their device would interact with an OLED display without having one on hand!

My inspiration for the project came from working on an IOT project and, halfway through, realizing I didn't have the display I thought I did. This bugged me as I have a computer 100x more powerful than the Arduino I was working with, and a nice monitor - why couldn't I just send and render the data that _would_ have gone to the small OLED display to my computer and render it there instead? ... turns out you can!

So if you don't have an OLED display, or just can't be bothered to find it, this emulator enables you to still run your program and see what the display would render!

## How to use

### Step 1
Download the executable directly from the `builds` folder [here](https://github.com/sam-peach/SSD1306-OLED-Emulator/tree/master/builds).
**Windows is only supported at the moment**

### Step 2
In your Arduino IDE, download the `Adafruit SSD1306 EMULATOR` library

![image](https://user-images.githubusercontent.com/47146346/187985507-382e9b46-a7d5-4fc6-aebe-15b3a05dc245.png)

### Step 3
Replace all occurrences of `#include <Adafruit_SSD1306>` with `#include <Adafruit_SSD1306_EMULATOR>` in your Arduino IDE file. Make sure to swap out all uses of the `Adafruit_SSD1306` type with `Adafruit_SSD1306_EMULATOR` as well.
```c++
//#include <Adafruit_SSD1306.h>
#include <Adafruit_SSD1306_EMULATOR.h>
```
```c++
//Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, OLED_RESET);
Adafruit_SSD1306_EMULATOR display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, OLED_RESET);
```

### Step 4
Set the baud rate in your Arduino IDE to `9600`

### Step 5
Flash the updated file onto your microcontroller

### Step 6
Run the display emulator executable downloaded in step 1 and pick the serial port your Arduino is connected to

![image](https://user-images.githubusercontent.com/47146346/187987118-c69a9fd4-d280-41e2-95ef-07926ab03dfc.png)

### Step 7 (Optional)
Reset your Arduino to sync the emulator and your device. The emulator will sync up on its own but may take a few frames.

![image](https://user-images.githubusercontent.com/47146346/187988028-fada03ae-00ef-4a2c-b69c-ce830682e43f.png)

## Compiling from source
With [Rust](https://www.rust-lang.org/) installed, this is as easy as cloning this repo and running `cargo run` in the cloned directory.

## Coming soon!
As this is still an early alpha build there are a few features that are still in the works:
- Test coverage: yeah, I know. I'll hand myself over to the TDD police.
- Scrolling: surprisingly fiddly to implement, but halfway there
- General stability: there are a few small things that currently crash the emulator, but should be recoverable instead.
