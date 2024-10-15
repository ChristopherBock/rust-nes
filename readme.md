# rust-nes

This is a simple Nintendo Entertainment System emulator written in rust.

# Why yet another NES emulator?

For the fun of learning rust by doing a project.

# What works?

* The CPU should be able to interpret and execute all opcodes correctly, including the illegal ones.
* There should be tests for all opcodes (but not necessarily all edge cases are well covered yet)
* The timing of the CPU is not yet correctly implemented.
* PPU and APU are not yet implemented.
* The workaround visualization from https://bugzmanov.github.io/nes_ebook/chapter_3_4.html is still in place and visualizing parts of the memory while e.g. the golden test is running.

rust-nes's CPU implementation has been tested and verified against http://nickmass.com/images/nestest.nes and an the corresponding log file https://www.qmtpro.com/%7Enes/misc/nestest.log . 

# Disclaimer

This software is without any warranty or the likes. It is provided as is.

# Pre-requisites

You have to have the development version of sdl2 installed. Additionaly you need to create a .env file from the .env.template template and adjust it to your needs.

# Usage

Currently it is assumed that only the debug version is used. No effort has been undertaken to make the buildscript deal with different build targets.

* "cargo test" will run the tests
* "cargo run" will run the emulator

Currently the emulator will load a file called "nestest.nes" and run it.
You can use the files from the nes_ebook ( https://bugzmanov.github.io/nes_ebook ) or the "golden sample" from http://nickmass.com/images/nestest.nes .
For the "golden sample" https://www.qmtpro.com/%7Enes/misc/nestest.log provides an instruction log similar to the one which rust-nes generates, so you can use it to verify the implementation.
The visualization from the snake example from https://bugzmanov.github.io/nes_ebook/chapter_5.html is still in place and still visualizes the workaround.

# Contributions

If you feel like contributing or have any improvement suggestions, feel free to give me a shout, I am happy to learn!

# Author

Christopher 'Alduin' Bock