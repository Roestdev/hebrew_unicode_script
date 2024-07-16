Hebrew_Unicode_Script
=
# Table of contents
1. [Description](#description)
2. [Install](#install)
3. [Unicode References](#unicode_ref)
4. [Safety](#safety)
5. [Panics](#panics)
6. [Errors](#errors)
7. [Code Coverage](#codecoverage)
8. [License](#license)


## Description <a name="description"></a>
This crate (*hebrew_unicode_script*) is a Rust library designed to facilitate the identification and validation of Unicode characters related to the Hebrew script and its associated blocks.   
This library provides a set of functions that allow developers to easily determine whether a particular character belongs to the *Hebrew unicode script*, falls within the *Hebrew unicode code block* or matches the *Alphabetic Presentation Form* unicode code block.   
For each of the applicable *unicode code blocks* there are some additional functions, allowing an even more sophisticated check for the *character type* within each code block. Examples include vowels, accents, marks, etc. etc.  
Each function in this library returns a boolean value, making it easy to integrate these checks into existing or new applications.

##### For unicode script (top level):
1. is_hebrew_script(c: char) -> bool

##### For unicode blocks (2nd level): 
1. is_hebrew_block(c: char) -> bool
2. is_hebrew_apf_block(c: char) -> bool

##### For unicode block *hebrew* (3rd level): 
1. is_accent(c: char) -> bool
2. is_mark(c: char) -> bool
3. is_vowel_point(c: char) -> bool
4. is_punctuation(c: char) -> bool
5. is_letter_normal(c: char) -> bool
6. is_letter_final(c: char) -> bool
7. is_letter(c: char) -> bool
8. is_yod_triangle(c: char) -> bool
9. is_ligature_yiddish(c: char) -> bool

##### For unicode block *Alphabetic Presentation Form* (3rd level): 
1. is_apf_point(c: char) -> bool
2. is_apf_letter(c: char) -> bool
3. is_apf_ligature_yiddisch(c: char) -> bool
4. is_apf_ligature(c: char) -> bool

## Install <a name="install"></a>
Run the following Cargo command in your project directory:
```
cargo add hebrew_unicode_script
```
**OR** add the following line to your Cargo.toml under **dependencies**
```
hebrew_unicode_script = "0.1.1"
```

## Examples <a name="examples"></a>
```rust
use hebrew_unicode_script::is_hebrew_block;
if is_hebrew_block('מ') {
	println!("The character you entered is part of the 'unicode code block Hebrew'");
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

```rust
use hebrew_unicode_script::*;

fn main() {
   // define a strings of characters
   let string_of_chars = "יָ֭דַעְתָּ שִׁבְתִּ֣י abcdefg וְקוּמִ֑י";
   // get a structures that indicates if a type is present or not (bool)
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

*Output result:*
   
``` text
The following character types were found:
HebrewCharacterTypes {
    accent: true,
    mark: false,
    vowel_point: true,
    punctuation: false,
    letter: true,
    letter_normal: true,
    letter_final: false,
    yod_triangle: false,
    ligature_yiddish: false,
    whitespace: true,
    non_hebrew: true,
}
```
See the crate modules for more examples.

## Unicode References <a name="unicode_ref"></a>
**Unicode Script**   
- [Hebrew](https://www.charactercodes.net/script/hebr)

**Unicode Block Names**
1. *Hebrew*
    - See <https://www.unicode.org/charts/PDF/U0590.pdf>
      - **Note:** only the following code-point range is applicable: *U+0590 .. U+05FF*
      - See also: <https://graphemica.com/blocks/hebrew/>  
2. *Alphabetic Presentation Form*
    - See <https://www.unicode.org/charts/PDF/UFB00.pdf> 
      - **Note:** only the following code-point range is applicable: *U+FB1D .. U+FB4F*
      - See also: <https://graphemica.com/blocks/alphabetic-presentation-forms>  

Learn more about [Unicode](https://www.unicode.org/), [Unicode scripts](https://www.unicode.org/standard/supported.html) and [Unicode code point blocks](https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt)


## Safety <a name="safety"></a>
All functions are written in safe Rust.

## Panics <a name="panics"></a>
Not that I am aware of.

## Errors <a name="errors"></a>
All functions return either true *or* false.

## Code Coverage <a name="codecoverage"></a>

Current code coverage is *99%* [^1]
[^1]: The code coverage figures shown in crates.io are (very) different! I don't know why :-) (WIP)

Based upon:  <https://blog.rng0.io/how-to-do-code-coverage-in-rust/#running-locally>

Actions:
1. Install the extension [Coverage Gutters](https://github.com/ryanluker/vscode-coverage-gutters)
2. Execute: *cargo clear && cargo build && cargo test*
3. Execute: *CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test*  
   - result -> (new file) cargo-test-67187-8558864636421498001_0.profraw
4. Execute: *grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/tests.lcov*
   - result -> (new file) tests.lcov
5. Click on the *Watch* button (added to VSCodium by the Ext)
   - result -> red/green indications will appear for each line of code
   
   OR 

4. Execute: *grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o html*
   - result -> a new directory called *html*
5. Open the file *index.html* in the folder html in your brower and you get a full report.

## License <a name="license"></a>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
