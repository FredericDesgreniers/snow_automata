# snow_automata

Attempt at a state machine generator using a nicer syntax than regular expressions in order to help re-make the parser for a compiler I'm working on

The goal of this project is to make several backends that compile the state machine into code for a couple of language and a regex backend

A minimal example of what the state machine definition looks like: 

```
start{
    'a'..'z' => identifier_state
    'A'..'Z' => identifier_state
    '0'..'9' => number_state
}

identifier_state{
    'a'..'z' => identifier_state
    'A'..'Z' => identifier_state
    '0'..'9' => identifier_state
    '_' => identifier_state
    _ => return IDENTIFIER
}

number_state{
    '0'..'9' => number_state
    '.' => float_state
    _ => return INTEGER
}

float_state{
    '0'..'9' => float_state
    'e' => exponential_float_state
    _ => return FLOAT
}

exponential_float_state{
    '0'..'9' => exponential_float_state
    _ => return FLOAT
}
```
