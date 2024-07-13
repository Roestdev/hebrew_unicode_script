Hebrew_Unicode_Script
=

This crate provides some functions that will check if a hebrew character belongs to a certain set inside the unicode script 'Hebrew'

## Examples

#### Simple

```rust
use hebrew_unicode_script::is_hebrew_block;

if is_hebrew_block(מ) {
	todo!()
}
```


```rust
use hebrew_unicode_script::is_letter_final;
use hebrew_unicode_script::is_letter;

let test_str = "ךםןףץ";
for c in test_str.chars() {
    assert!(is_letter_final(c));
    assert!(is_letter(c));
}
```
### More complex


```rust
use hebrew_unicode_script::*;

fn main() {
   // define a strings of characters
   let string_of_chars = "אַתָּ֣ה יָ֭דַעְתָּ שִׁבְתִּ֣י וְקוּמִ֑י בַּ֥נְתָּהg p לְ֝רֵעִ֗י מֵרָחֽוֹק׃";
   // get a structures that indicates if a typeis present or not (bool)
   let chartypes = get_character_types(string_of_chars);
   // print the results
   println!("The following letter types are found in: {}", string_of_chars);
   println!("{:?}",chartypes);
}

#[derive(Debug, Default)]
pub struct HebrewCharacterTypes {
    accent: bool,
    mark: bool,
    vowel_point: bool,
    punctuation: bool,
    letter: bool,
    letter_normal: bool,
    letter_final: bool,
    yod_triangle: bool,
    ligature_yiddish: bool,
    whitespace: bool,
    non_hebrew: bool,
}

impl HebrewCharacterTypes {
    fn new() -> Self {
        Default::default()
    }
}

pub fn get_character_types(s: &str) -> HebrewCharacterTypes {
    let mut found_character_types = HebrewCharacterTypes::new();
    for c in s.chars() {
        match c {
            c if is_accent(c) => found_character_types.accent = true,
            c if is_mark(c) => found_character_types.mark = true,
            c if is_vowel_point(c) => found_character_types.vowel_point = true,
            c if is_punctuation(c) => found_character_types.punctuation = true,
            c if is_letter_final(c) => found_character_types.letter_final = true,
            c if is_letter_regular(c) => found_character_types.letter_normal = true,
            c if is_yod_triangle(c) => found_character_types.yod_triangle = true,
            c if is_ligature_yiddish(c) => found_character_types.ligature_yiddish = true,
            c if c.is_whitespace() => found_character_types.whitespace = true,
            _ => found_character_types.non_hebrew = true,
        }
    }
    found_character_types.letter =
        found_character_types.letter_normal | found_character_types.letter_final;
    found_character_types
}
```

*Output:*
   
```
The following letter types are found in: אַתָּ֣ה יָ֭דַעְתָּ שִׁבְתִּ֣י וְקוּמִ֑י בַּ֥נְתָּהg p לְ֝רֵעִ֗י מֵרָחֽוֹק׃
HebrewCharacterTypes {
     accent: true,
     mark: false,
     vowel_point: true,
     punctuation: true,
     letter: true,
     letter_normal: true,
     letter_final: false,
     yod_triangle: false,
     ligature_yiddish: false,
     whitespace: true,
     non_hebrew: true,
}
```

### Panics
Not that I am aware of.

### Errors
All functions return either true *or* false.

### Safety
All functions are written in safe Rust.

## References

[**Unicode script for Hebrew**](https://www.charactercodes.net/script/hebr)   
- More elaborated overiew of [Supported scripts in unicode](https://www.unicode.org/standard/supported.html)
 
[**Unicode Block Names**](https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt) that contain Hebrew characters:
1. [*Hebrew*](https://www.unicode.org/charts/PDF/U0590.pdf)  
    - See also: <https://graphemica.com/blocks/hebrew/>  
2. [*Alphabetic Presentation Form*](https://www.unicode.org/charts/PDF/UFB00.pdf)  
    - See also: <https://graphemica.com/blocks/alphabetic-presentation-forms>  

## License
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
