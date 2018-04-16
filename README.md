# snow_automata

Attempt at a state machine generator using a nicer syntax than regular expressions in order to help re-make the parser for a compiler I'm working on

The goal of this project is to make several backends that compile the state machine into code for a couple of language and a regex backend

A minimal example of what the state machine definition looks like: 

```
start{
    'a'..'z' | '_' => identifier_state
    '0'..'9' => number_state
    "Self" =>  return KEYWORD_SELF
}

identifier_state{
    'a'..'z' | '0'..'9' | '_'  => Self
    _ => return IDENTIFIER
}

number_state{
    '0'..'9' => Self
    '.' => float_state
    _ => return NUMBER
}

float_state{
    '0'..'9' => Self
    _ => return FLOAT
}

```
