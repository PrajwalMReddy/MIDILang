# The MIDILang Syntax Guide
This syntax guide shows the different syntactic features of the latest version of MIDILang (Version 0.1.0). This guide is not a specification but a reference that highlights the full range of syntactic features available in MIDILang.

## Comments
```midilang
// Single line comments are created using a double slash.
// Multiline comments are not supported.
```

## Operators
### Binary Operations
```midilang
a + 10; // Binary Addition
a - 10; // Binary Subtraction
a * 10; // Binary Multiplication
a / 10; // Binary Division
```

## Note Statements
```midilang
note 10 20 30; // Plays a note of pitch 30, duration 20, and loudness 30.
```

## Tune Statements
```midilang
play JingleBells; // Plays the tune named 'JingleBells'.
```

## Variables
```midilang
var a = 100; // Declares a and initializes it to 100.
a = 127; // Assigns the value 10 to the variable a.
```
Only the int datatype is available for variables.

## Loops
```midilang
loop: 10 {
    note 100 100 100;
}
```
The number of iterations, in this case '10', is given after the colon.

## Declaring Tunes
```midilang
// Tune test1 does not take any arguments.
tune test1 {
    note 60 196 64;
}

// Tune test2 takes 'duration' and 'volume' as arguments.
tune test2: duration volume {
    note 60 duration volume;
}
```
Tunes can take in arguments, which are written after a colon. If there are no arguments, there should not be a colon.
