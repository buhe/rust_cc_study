### Constraint
1. 只在终结符号移动下标，非终结不移动。
### step1
```
program
    : function

function
    : type Identifier '(' ')' '{' statement '}'

type
    : 'int'

statement
    : 'return' expression ';'

expression
    : Integer
```

### step2
```
expression
    : unary

unary
    : Integer
    | ('-'|'!'|'~') unary
```
- 一元运算符是右结合. 在 IR 阶段实现
#### ir

_T0 = 1
_T1 = NEG _T0

### step2.5
```
unary
    : primary
    | ('-'|'~'|'!') unary

primary
    : Integer
    | '(' expression ')'
```
### step3
```
expression
    : additive

additive
    : multiplicative
    | additive ('+'|'-') multiplicative

multiplicative
    : unary
    | multiplicative ('*'|'/'|'%') unary

unary
    : primary
    | ('-'|'~'|'!') unary

primary
    : Integer
    | '(' expression ')'
```

#### step3 消除左递归后
```
A = Aa | Ab | r
消除后
A = rT
T = aT | bT | <
```
```
expression
    : additive

additive
    : multiplicative rest
rest
    : ('+'|'-') multiplicative rest
    | <
multiplicative
    : unary rest2
rest2
    : ('*'|'/'|'%') unary rest2
    | <
unary
    : primary
    | ('-'|'~'|'!') unary

primary
    : Integer
    | '(' expression ')'
```
#### ast
```
expression
    : unary
    | expression + expression
    | expression * expression

```
#### ir
```
_T0 = 1
_T1 = 3
_T2 = ADD _T0, _T1
_T3 = SUB _T0, _T1
_T4 = MUL _T0, _T1
_T5 = DIV _T0, _T1
_T6 = MOD _T0, _T1
```

### step4
```
expression
    : logical_or

logical_or
    : logical_and
    | logical_or '||' logical_and

logical_and
    : equality
    | logical_and '&&' equality

equality
    : relational
    | equality ('=='|'!=') relational

relational
    : additive
    | relational ('<'|'>'|'<='|'>=') additive
```
#### step4 消除左递归
```
A = Aa | Ab | r
消除后
A = rT
T = aT | bT | <
```
```
logical_or
    : logical_and rest3

rest3
    : '||' logical_and rest3
    | <

logical_and
    : equality rest4

rest4
    : '&&' equality rest4
    | <

equality
    : relational rest5

rest5
    : ('=='|'!=') relational rest5
    | <

relational
    : additive rest6

rest6
    : ('<'|'>'|'<='|'>=') additive rest6
    | rest6
```
#### ir
```
_T0 = 1
_T1 = 3
_T2 = GT _T0, _T1
_T3 = GET _T0, _T1
_T4 = LT _T0, _T1
_T5 = LET _T0, _T1
_T6 = AND _T0, _T1
_T7 = OR _T0, _T1
_T8 = EQUAL _T0, _T1
_T9 = NOT_EQUAL _T0, _T1
```
#### asm
```

```

### step5
```
program
    : function

function
    : type Identifier '(' ')' '{' statement* '}'

type
    : 'int'

statement
    : 'return' expression ';'
    | expression? ';'
    | declaration

declaration
    : type Identifier ('=' expression)? ';'

expression
    : assignment

assignment
    : logical_or
    | Identifier '=' expression

logical_or
    : logical_and
    | logical_or '||' logical_and

logical_and
    : equality
    | logical_and '&&' equality

equality
    : relational
    | equality ('=='|'!=') relational

relational
    : additive
    | relational ('<'|'>'|'<='|'>=') additive

additive
    : multiplicative
    | additive ('+'|'-') multiplicative

multiplicative
    : unary
    | multiplicative ('*'|'/'|'%') unary

unary
    : primary
    | ('-'|'~'|'!') unary

primary
    : Integer
    | '(' expression ')'
    | Identifier
```

### step6
```
program
    : function

function
    : type Identifier '(' ')' '{' block_item* '}'

type
    : 'int'

block_item
    : statement
    | declaration

statement
    : 'return' expression ';'
    | expression? ';'
    | 'if' '(' expression ')' statement ('else' statement)?

declaration
    : type Identifier ('=' expression)? ';'

expression
    : assignment

assignment
    : conditional
    | Identifier '=' expression

conditional
    : logical_or
    | logical_or '?' expression ':' conditional

logical_or
    : logical_and
    | logical_or '||' logical_and

logical_and
    : equality
    | logical_and '&&' equality

equality
    : relational
    | equality ('=='|'!=') relational

relational
    : additive
    | relational ('<'|'>'|'<='|'>=') additive

additive
    : multiplicative
    | additive ('+'|'-') multiplicative

multiplicative
    : unary
    | multiplicative ('*'|'/'|'%') unary

unary
    : primary
    | ('-'|'~'|'!') unary

primary
    : Integer
    | '(' expression ')'
    | Identifier
```
#### ir
```
main:
    _T1 = 1
    _T0 = _T1
    BEQZ _T0, _L1
    _T2 = 2
    _T0 = _T2
    JUMP _L2
_L1:
    _T3 = 3
    _T0 = _T3
_L2:
    return _T0
```
#### asm
```
step6:             # RISC-V 汇编标签
    beqz t1, step6 # 如果t1为0，跳转到 step6 标签处
    j step6        # 无条件跳转到 step6 标签处
```