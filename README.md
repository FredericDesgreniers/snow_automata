# snow_automata

Attempt at a state machine generator using a nicer syntax than regular expressions in order to help re-make the parser for a compiler I'm working on

The goal of this project is to make several backends that compile the state machine into code for a couple of language and a regex backend

A minimal example of what the state machine definition looks like: 

```
start{
    'a'..'z' => identifier_state
    'A'..'Z' => identifier_state
    '0'..'9' => number_state
    "=>" => return ARROW
    "Self" =>  return KEYWORD_SELF
    "return" => return KEYWORD_RETURN
    '{' => return BRACKET_OPEN
    '}' => return BRACKET_CLOSE
}

identifier_state{
    'a'..'z' => Self
    'A'..'Z' => Self
    '0'..'9' => Self
    '_' => Self
    _ => return IDENTIFIER
}

number_state{
    '0'..'9' => Self
    '.' => float_state
    _ => return INTEGER
}

float_state{
    '0'..'9' => Self
    'e' => exponential_float_state
    _ => return FLOAT
}

exponential_float_state{
    '0'..'9' => Self
    _ => return FLOAT
}
```
