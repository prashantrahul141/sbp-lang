### Grammer
```
expression  ->   literal | unary | binary | grouping ;

literal     ->   NUMBER | STRING | "true" | "false" | "null" ;

grouping    ->   "(" expression ")" ;

unary       ->   ( "-" | "!" ) expression ;

binary      ->   expression operator expression ;

operator    ->   "==" | "!=" | "<" | "<=" | ">" | ">=" | "+"  | "-"  | "*" | "/" ;
```

### Recursive descent parser grammer
```
program       ->     declaration* EOF ;

declaration   ->     letDecl
                    | statement ;

letDecl       ->     "let" IDENTIFIER ( "=" expression )? ";" ;

statement     ->     exprStmt
                   | forStmt
                   | ifStmt
                   | printStmt
                   | whileStmt
                   | block ;

forStmt       ->     "for" "(" (letDecl | exprStmt | ";" ) expression? ";" expression? ")" statement ;

whileStmt     ->     "while" "(" expression ")" statement ;

ifStmt        ->     "if" "(" expression ")" statement
                     ( "else" statement )? ;

block         ->     "{" declaration* "}" ;

expression    ->     assignment;

assignment    ->     INDENTIFIER "=" assignment | logic_or ;

logic_or      ->     logic_and ( "or" logic_and )* ;

logic_and     ->     equality ( "and" equality )* ;

equality      ->     comparison ( ( "!=" | "==" ) comparison )* ;

comparison    ->     term ( ( ">" | ">=" | "<" | "<=" ) term )* ;

term          ->     factor ( ( "-" | "+" ) factor )* ;

factor        ->     unary ( ( "/" | "*" ) unary )* ;

unary         ->     ( "!" | "-" ) unary | call ;

call          ->     primary ( "(" arguments? ")" )* ;

arguments     ->     expression ( "," expression )* ;

primary       ->     NUMBER 
                   | STRING 
                   | "true" 
                   | "false" 
                   | "null" 
                   | "(" expression ")" 
                   | IDENTIFIER ;

```

### Some clarifications

- CAPITAL case words are values of the type described by the word
- lower case words are **Non-terminal Symbol**
- quoted strings are **Terminal Symbols**
- `->` indentifier before it defines the rule name, after it defines the rule's body.
- `;` marks end of a rule
- `|` is OR
- `(` and `)` for grouping
- `*` previous item can appear zero or multiple times
- `+` previous item can appear atleast once
- `?` previous item can appear zero or one time, but not more

**Terminal Symbol** : A terminal is a letter from the grammar’s alphabet. You can think of it like a literal value. In the syntactic grammar we’re defining, the terminals are individual lexemes—tokens coming from the scanner like if or 1234.

**Non-Terminal Symbol** : A nonterminal is a named reference to another rule in the grammar. It means “play that rule and insert whatever it produces here”. In this way, the grammar composes.

