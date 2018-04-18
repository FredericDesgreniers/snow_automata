# snow_automata

Attempt at a state machine generator using a nicer syntax than regular expressions in order to help re-make the parser for a compiler I'm working on

The goal of this project is to make several backends that compile the state machine into code for a couple of language and a regex backend

A minimal example of what the state machine definition looks like: 

```
state start {
    'a'..'z' | '_' => identifier
    '0'..'9' => number
    "Self" =>  return KEYWORD_SELF
}

state identifier {
    'a'..'z' | '0'..'9' | '_'  => Self
    _ => return IDENTIFIER
}

state number {
    '0'..'9' => Self
    '.' => float
    _ => return NUMBER
}

state state {
    '0'..'9' => Self
    _ => return FLOAT
}
```
