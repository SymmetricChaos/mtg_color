# mtg_color

Do you absolutely need to be able to represent the colors of Magic the Gathering cards (or other game objects) in a single byte? Is maximum space efficiency non-negotiable for some reason? Then this is the crate for you!

Un-set colors are not supported.

Features:  
* no-std compatible  
* zero dependencies  
* coded in pure Rust
* all mana symbol information stored in 14 bytes, less than a fifth the space used by naïve competitors (NOW PROVEN OPTIMAL)