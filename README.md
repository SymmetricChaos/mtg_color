# mtg_color

Do you absolutely need to be able to represent the colors of Magic the Gathering cards (or other game objects) in a single byte? Are ergonmics irrelevant and storage space non-negotiable?

Features:  
* no-std compatible  
* only 14 bytes shared bytes needed for all canonical symbol information (compared to 80 bytes for a naive implementation)  
* already at a stable 1.0.0 release  
* zero dependencies  
* coded in pure Rust for speed and safety