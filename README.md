# L1IR
Intermediate Representation of [LSTS](https://github.com/andrew-johnson-4/LSTS) [L1 Language](https://github.com/andrew-johnson-4/LSTS/blob/main/preludes/l1.tlc)

Types
* Literal Strings
* Tuples
* Functions

Global AST Nodes
* Function Definitions
* Program Expressions

Expression AST Nodes
* Literal Introduction
* Tuple Introduction
* Variable Reference
* Function Application
* Pattern Match
* Program Failure, Immediate Exit with possible Message

L1IR's unique contribution is that it does not presume to know everything about literal strings. Types, by definition, are represented as an amalgam of Unicode Characters instead of fixed length bitstrings. This is advantageous to languages like L1 that define their own operators from scratch, but still desire to have an efficient runtime.
