<!-- This doc shall contain the language syntax and specification -->
<!-- This doc will be heavily changed for the upcoming iterations -->

# Cleva


## Features
- **Typed-** All variables must be statically typed. 
- **Expression Oriented-** Every(or maybe, almost every) statement is an expression. Cleva might not be object oriented, but it is expression oriented.
- **Crisp Syntax-** The syntax of cleva is meant to typed as fast as possible. Hence it will be as short as a scripting language as python. _Suggestions to change the surface sytax are always welcomed!_
    - **Variable Declarations:** As short as `a = 5`. The type of variable will be inferred. Defaults will be chosen as per some standards. If types need to be specified, they can be done as `a\i32 = 5`.
    Multiple declarations can be done simply as `a\u32, b\i32 = 0,0`
- **Semi-first Order functions-** Any function may itself be an almost first class citizen. I donot intend to make Cleva a functional programming language in itself, since it compiles to C. But all my favorite features of functional programming shall be included.
- **STL, batteries included for competive programming-** The STL provided with Cleva should be sufficient to solve any competive programming problem.
- **Helpful Debugging-** The error messages should be meaningful enough to point out a mathematical flaw in your design, rather than the compiler being dumb to respect your code.
- **Low Cost Abstraction-** The final outputted C source code should be just as fine as if you had natively implemented the code in C. 


see also from here
- [STL](./STL.md)