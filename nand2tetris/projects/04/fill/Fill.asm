// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed.
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

@LOOP
    @KBD
    D = M

    @i
    M = 0
    @j
    M = 0

    @KEYON
    0; JNE
    @KEYOFF
    D; JEQ

    @KEYON
        @KEYONLOOP
            @i
            D = M
            @131072
            D = A - D
            @LOOP
            D; JEQ

            @SCREEN
            D = M
            @i
            A = D + M
            M = 1

            @i
            M = M + 1
            @KEYONLOOP
            0; JMP

    @KEYOFF
        @KEYOFFLOOP
            @j
            D = M
            @131072
            D = A - D
            @LOOP
            D; JEQ

            @SCREEN
            D = M
            @j
            A = D + M
            M = 0

            @j
            M = M + 1
            @KEYOFFLOOP
            0; JMP

@LOOP
0; JEQ
