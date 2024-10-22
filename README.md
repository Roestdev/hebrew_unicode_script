# Hebrew_Unicode_Script
![Crates.io License](https://img.shields.io/crates/l/hebrew_unicode_script)
![Crates.io Version](https://img.shields.io/crates/v/hebrew_unicode_script)
![Static Badge](https://img.shields.io/badge/no--std-yes-00FFFF)
![docs.rs](https://img.shields.io/docsrs/hebrew_unicode_script)
[![Build & Test](https://github.com/Roestdev/hebrew_unicode_script/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/Roestdev/hebrew_unicode_script/actions/workflows/build_and_test.yml)
[![Clippy Analyze](https://github.com/Roestdev/hebrew_unicode_script/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Roestdev/hebrew_unicode_script/actions/workflows/rust-clippy.yml)

## Table of contents <a name="toc"></a>
- [Hebrew\_Unicode\_Script](#hebrew_unicode_script)
  - [Table of contents ](#table-of-contents-)
  - [Description ](#description-)
  - [Function overview](#function-overview)
    - [Unicode script 'Hebrew' (top level):](#unicode-script-hebrew-top-level)
    - [Unicode block: 'Hebrew'](#unicode-block-hebrew)
    - [Unicode block: 'Alphabetic Presentation Form'](#unicode-block-alphabetic-presentation-form)
  - [Examples ](#examples-)
    - [Using the function API](#using-the-function-api)
    - [Using the trait API](#using-the-trait-api)
  - [no\_std](#no_std)
  - [Install ](#install-)
  - [Safety ](#safety-)
  - [Panics ](#panics-)
  - [Errors ](#errors-)
  - [Code Coverage ](#code-coverage-)
  - [License ](#license-)
  - [Contribution ](#contribution-)
  - [Notes](#notes)
    - [Points](#points)
    - [Letters](#letters)
    - [Accents](#accents)
    - [Hebrew Disjunctive and Conjunctive Accents: Understanding Their Role](#hebrew-disjunctive-and-conjunctive-accents-understanding-their-role)
      - [Disjunctive Accents](#disjunctive-accents)
      - [Conjunctive Accents](#conjunctive-accents)
      - [Examples of Disjunctive and Conjunctive Accents](#examples-of-disjunctive-and-conjunctive-accents)
      - [Importance in Modern Hebrew](#importance-in-modern-hebrew)
  - [References ](#references-)
    - [Unicode Script](#unicode-script)
    - [Unicode Block Names](#unicode-block-names)
    - [Unicode Problems for Hebrew](#unicode-problems-for-hebrew)
  - [Hebrew accents](#hebrew-accents)
    - [Introduction](#introduction)
    - [Disjunctive Accents in Prose Books](#disjunctive-accents-in-prose-books)
    - [Disjunctive Accents in Non-Prose Books](#disjunctive-accents-in-non-prose-books)
    - [Conjunctive Accents in Prose and Poetry](#conjunctive-accents-in-prose-and-poetry)
    - [Significance of Accent Differences](#significance-of-accent-differences)
    - [Conclusion](#conclusion)


## Description <a name="description"></a>

This crate (`hebrew_unicode_script`) is a low level library written in Rust and designed to facilitate the identification and validation of Unicode characters related to the Hebrew script and its associated unicode code blocks.   

This library provides *two* types of interface:
 1. **functions**
 2. **trait** (the same functions behind one trait)

The given set of functions (either direct of via a trait) allow developers to easily determine whether a particular character belongs to the *Hebrew unicode script*, falls within the *Hebrew unicode code block* or matches the *Alphabetic Presentation Form* unicode code block.   

For both of the applicable *unicode code blocks* there are some additional functions, allowing an even more refined check for the *character type* within each code block. Examples include vowels, accents, marks, etc.  

Each function in this library returns a boolean value, making it easy to integrate these checks into existing or new applications.

For an overview of released versions see [releases](https://github.com/Roestdev/hebrew_unicode_script/releases).

## Function overview

### Unicode script 'Hebrew' (top level):

``` sh
  - is_script_hbr(c: char) -> bool
  - is_script_hbr_point(c: char) -> bool
  - is_script_hbr_consonant(c: char) -> bool
  - is_script_hbr_ligature_yiddisch(c: char) -> bool
```

### Unicode block: 'Hebrew' 

2nd level:
``` sh
  - is_hbr_accent(c: char) -> bool
  - is_hbr_mark(c: char) -> bool
  - is_hbr_point(c: char) -> bool
  - is_hbr_punctuation(c: char) -> bool
  - is_hbr_consonant(c: char) -> bool
  - is_hbr_yod_triangle(c: char) -> bool
  - is_hbr_ligature_yiddish(c: char) -> bool
```

3rd level:
``` sh
    - is_hbr_point_vowel(c) -> bool
    - is_hbr_point_semi_vowel(c) -> bool
    - is_hbr_point_reading_sign(c) -> bool
    - is_hbr_consonant_normal(c: char) -> bool
    - is_hbr_consonant_final(c: char) -> bool
```

### Unicode block: 'Alphabetic Presentation Form'

2nd level:
``` sh
  - is_apf_block(c: char) -> bool
  - is_apf_point_reading_sign(c: char) -> bool
  - is_apf_consonant(c: char) -> bool
  - is_apf_ligature_yiddisch(c: char) -> bool
  - is_apf_ligature(c: char) -> bool
```

3rd level:
``` sh
    - is_apf_consonant_alternative(c: char) -> bool
    - is_apf_consonant_wide(c: char) -> bool
    - is_apf_consonant_with_vowel(c: char) -> bool
```

## Examples <a name="examples"></a>

### Using the function API

Basic usage:

```rust
use hebrew_unicode_script::is_hbr_block;

if is_hbr_block('מ') {
	println!("The character you entered is part of the 'unicode code block Hebrew'");
}
```

```rust
use hebrew_unicode_script::{is_hbr_consonant_final, is_hbr_consonant};

let test_str = "ךםןףץ";
for c in test_str.chars() {
    assert!(is_hbr_consonant_final(c));
    assert!(is_hbr_consonant(c));
}
```

A more complex example:
```rust
use hebrew_unicode_script::{is_hbr_accent,is_hbr_mark, is_hbr_point, is_hbr_punctuation};
use hebrew_unicode_script::{is_hbr_consonant_final,is_hbr_yod_triangle,is_hbr_ligature_yiddish};

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
    point: bool,
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
            c if is_hbr_accent(c) => found_character_types.accent = true,
            c if is_hbr_mark(c) => found_character_types.mark = true,
            c if is_hbr_point(c) => found_character_types.point = true,
            c if is_hbr_punctuation(c) => found_character_types.punctuation = true,
            c if is_hbr_consonant_final(c) => found_character_types.letter_final = true,
            c if is_hbr_yod_triangle(c) => found_character_types.yod_triangle = true,
            c if is_hbr_ligature_yiddish(c) => found_character_types.ligature_yiddish = true,
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
   
``` txt
The following character types were found:
HebrewCharacterTypes {
    accent: true,
    mark: false,
    point: true,
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

### Using the trait API

```
use hebrew_unicode_script::HebrewUnicodeScript;

assert!( 'מ'.is_script_hbr() );
assert!( !'מ'.is_script_hbr_point() );
assert!( 'מ'.is_script_hbr_consonant() );
assert!( 'ױ'.is_script_hbr_ligature_yiddisch() );
assert!( 'מ'.is_hbr_block() );
assert!( !'מ'.is_hbr_accent() );
assert!( !'מ'.is_hbr_mark() );
assert!( !'מ'.is_hbr_point() );

```

See the crate modules for more examples.

## no_std

`hebrew_unicode_script` does not depend on a standard library, nor a system allocator.

[^ TOC](#toc)

## Install <a name="install"></a>

For installation see the [hebrew_unicode_script](https://crates.io/crates/hebrew_unicode_script) page at crates.io.

[^ TOC](#toc)

## Safety <a name="safety"></a>

All functions are written in safe Rust.

[^ TOC](#toc)

## Panics <a name="panics"></a>

Not that I am aware of.

[^ TOC](#toc)

## Errors <a name="errors"></a>

All (trait)functions return either true *or* false.

[^ TOC](#toc)

## Code Coverage <a name="codecoverage"></a>

Current code coverage is *100%* [^code coverage]
[^code coverage]: The code coverage figures shown in crates.io are (very) different!

![Code Coverage](doc/images/Code_Coverage_20240827_09-44.png)


I used [code coverage, running locally](https://blog.rng0.io/how-to-do-code-coverage-in-rust/#running-locally)

Actions:
1. Install the extension [Coverage Gutters](https://github.com/ryanluker/vscode-coverage-gutters)
2. Execute: `cargo clean && mkdir -p target/coverage/html`
3. Execute: `CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test`
   - result -> (new file) cargo-test-67187-8558864636421498001_0.profraw (on my system)

Option 1: Using Coverage Gutters

1. Execute: `grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/tests.lcov`
   - result -> (new file) tests.lcov
  
2. Click on the *Watch* button (added to VSCodium by the Ext)
   - result -> red/green indications will appear for each line of code
   
Option 2: Creating a webpage

1. Execute: `grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o target/coverage/html`
    - result -> a new directory called *html*
2. Open the file *index.html* in the folder html in your brower and you get a full report.


[^ TOC](#toc)

## License <a name="license"></a>

Licensed under either of 
<a href="LICENSE-APACHE">Apache License, Version2.0</a> 
or 
<a href="LICENSE-MIT">MIT license</a> at your option.


[^ TOC](#toc)

## Contribution <a name="contribution"></a>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[^ TOC](#toc)

## Notes

### Points

- Hebrew **points** can be subdivided in:
  - Vowels (code points: *U+05B4 .. U+05BB, U+05C7*)
  - Semi-Vowels (code points: *U+05B0 .. U+05B3*)  
  - Reading Signs (code points: *U+05BC .. U+05BD + U+05BF + U+05C1 .. U+05C2*) [^judeo-spanish]
  [^judeo-spanish]: For me it not clear yet if the 'HEBREW POINT JUDEO-SPANISH VARIKA' a reading sign or not. For the time being this code-point will be part of the *reading signs*

### Letters

- Hebrew **letters** (consonants) can be subdivided in:
  - Normal consonants (code points: *U+05D0 .. U+05D9, U+05DB, U+05DC, U+05DE, U+05E0 .. U+05E2, U+05E4, U+05E6 .. U+05EA*)
  - Final consonants (code points: *U+05DA, U+05DD, U+05DF, U+05E3 and U+05E5*)  
  - Wide consonants (code points: *U+FB21 .. U+FB28*)
  - Consonants with vowel (code points: *U+FB2A .. U+FB36, U+FB38 .. U+FB3C, U+FB3E, U+FB40, U+FB41, U+FB43, U+FB44, U+FB46 .. U+FB4E*)
  - Alternative consonants (code points: *U+FB20, U+FB29*)

### Accents

Accents Prose Conjuntive
Accents Prose Disjunctive

Accents Poetic Conjuntive
Accents Poetic Disjunctive


### Hebrew Disjunctive and Conjunctive Accents: Understanding Their Role

Hebrew, like many other Semitic languages, uses a system of diacritical marks called accents to indicate pronunciation and
 grammatical function. The two main types of accents used in Hebrew are disjunctive and conjunctive accents. These accents
 play crucial roles in determining word stress, syllable division, and even the meaning of certain words.

#### Disjunctive Accents

Disjunctive accents are used to separate words within a phrase or sentence. They serve several purposes:

1. Word Separation: 
   - Disjunctive accents mark the beginning of each word in a phrase.
   - This helps readers identify where one word ends and another begins.

2. Stress Indication:
   - In some cases, disjunctive accents can indicate primary stress within a word.
   - For example, the disjunctive accent on the first syllable of "הַיּוֹם" (hayom) indicates that this is the stressed syllable.

3. Syllable Division:
   - Disjunctive accents can sometimes affect how a word is divided into syllables.
   - This is particularly important when reading aloud, as it affects the natural flow of speech.

#### Conjunctive Accents

Conjunctive accents, on the other hand, connect words together within a phrase or sentence. They have several functions:

1. Connecting Words:
   - Conjunctive accents link words that are closely related semantically or grammatically.
   - They show how words should be grouped together for proper interpretation.

2. Stress Distribution:
   - Conjunctive accents can distribute stress across connected words.
   - This helps maintain a natural rhythm in spoken Hebrew.

3. Meaning Alteration:
   - In some cases, the presence or absence of a conjunctive accent can change the meaning of a word or phrase.
   - For instance, the addition of a conjunctive accent can turn a noun into a verb.

#### Examples of Disjunctive and Conjunctive Accents

Let's examine some examples to illustrate the use of these accents:

1. Disjunctive Accent Example:
   - הַיּוֹם הַשְּׁבִי (hayom hashvi'i)
     - Here, the disjunctive accents separate the words and indicate the stress pattern.

2. Conjunctive Accent Example:
   - בָּנָה אֶת הָאֳהָל (banah et ha'ohal)
     - The conjunctive accent connects the words while indicating the stress distribution.

3. Accent Change in Meaning:
   - מָכִיר (makhir) means "to recognize"
   - מָכִיר (makhir) without the conjunctive accent means "to sell"

#### Importance in Modern Hebrew

While modern Hebrew has largely abandoned the use of these accents in everyday writing, they remain crucial for:

1. Proper Pronunciation:
   - Native speakers still rely on accents to guide correct pronunciation.

2. Reading Ancient Texts:
   - Scholars studying ancient Hebrew texts must understand these accents to accurately interpret the original text.

3. Biblical Study:
   - In biblical studies, understanding the accents is essential for accurate translation and interpretation.

In conclusion, the disjunctive and conjunctive accents in Hebrew are more than just decorative marks. They carry significant
 weight in determining word stress, syllable division, and even the meaning of certain words. While their usage may vary
 depending on the context and historical period, understanding these accents remains vital for anyone interested in the
 intricacies of the Hebrew language.



[^ TOC](#toc)

## References <a name="references"></a>

### Unicode Script  

- [Unicode script for the Hebrew language](https://www.charactercodes.net/script/hebr)

### Unicode Block Names

1. *Hebrew*
    - See <https://www.unicode.org/charts/PDF/U0590.pdf>
      - **Note:** only the following code-point range is applicable: *U+0590 .. U+05FF*
      - See also: <https://graphemica.com/blocks/hebrew/>  
2. *Alphabetic Presentation Form*
    - See <https://www.unicode.org/charts/PDF/UFB00.pdf> 
      - **Note:** only the following code-point range is applicable: *U+FB1D .. U+FB4F*
      - See also: <https://graphemica.com/blocks/alphabetic-presentation-forms>  

Learn more about [Unicode](https://www.unicode.org/), [Unicode scripts](https://www.unicode.org/standard/supported.html) and [Unicode code point blocks](https://www.unicode.org/Public/UCD/latest/ucd/Blocks.txt)

### Unicode Problems for Hebrew

There are some issues with unicode and Hebrew. These are described on the following web page: [Unicode Problems](https://mechon-mamre.org/c/hr/unicode.htm)  

## Hebrew accents

[^ TOC](#toc)

### Introduction

The Old Testament, also known as the Hebrew Bible or Tanach, contains both prose and non-prose (poetic) texts. These
 different literary forms often exhibit distinct patterns in their use of Hebrew accents, including disjunctive (dagesh) and
 conjunctive (shva) accents. Understanding these differences can provide valuable insights into the structure and meaning of
 various biblical books.

### Disjunctive Accents in Prose Books

Prose books in the Old Testament generally employ disjunctive accents differently compared to poetic works:

1. More frequent use: Dageshes tend to be more common in prose due to the need for clear word boundaries in continuous narrative.

2. Consistent placement: Dageshes are typically placed consistently at the beginning of words, creating a rhythmic pattern that aids readability.

3. Clause separation: Multiple dageshes may be used more frequently to separate clauses within longer sentences.

Examples from prose books:

``` text
וַיֹּאמֶר מֹשֶׁה אֵלָיו כֹה עֲשִׂית לְךָ וְעָשִׂית לְךָ חֹבֶס וְנָחָתָה בּוֹ וְנָחָתָה בּוֹ וְנָחָתָה בּוֹ (va-yomer Moshe
 elav koh asit lekha v'asit lekha chovesh v'nachatah bo)
```

This excerpt from Exodus 25:9 demonstrates consistent dagesh placement at word beginnings.

### Disjunctive Accents in Non-Prose Books

Non-prose books, particularly poetic works, often exhibit different patterns in disjunctive accent usage:

1. Less frequent overall: While still present, dageshes are generally less common due to the natural rhythm of poetry.

2. Stress emphasis: Dageshes may be placed more strategically to emphasize certain syllables or words.

3. Metrical considerations: Accent patterns may be adjusted to fit specific metrical schemes or poetic devices.

Examples from poetic books:

``` text
הַמֶּלֶךְ הָאֹתִיּוֹת (ha-melekh ha-otiyot)
```

This phrase from Isaiah 8:1 uses a single dagesh to separate words, creating a rhythmic effect suitable for poetic
 expression.

### Conjunctive Accents in Prose and Poetry

Conjunctive accents (shvas) tend to follow similar patterns in both prose and poetic texts:

1. Word connection: Shvas are used consistently to connect words, regardless of literary form.

2. Enclitic pronouns: Their use for attaching possessive pronouns remains constant across genres.

3. Elision indication: Shvas indicating vowel omission are similarly employed in both prose and poetry.

However, poetic texts may occasionally use shvas in creative ways to enhance rhythm or meter.

### Significance of Accent Differences

Understanding these differences in accent usage between prose and poetic texts offers several insights:

1. Literary analysis: Recognizing accent patterns aids in identifying the genre of a text and analyzing its literary structure.

2. Interpretation guidance: Proper identification of accents can guide readers interpreting complex passages more accurately.

3. Pronunciation clues: Accent patterns provide valuable cues for correct pronunciation, especially in cases where vowel points alone might not be sufficient.

4. Textual criticism: Differences in accent usage can be useful tools in textual analysis, potentially revealing variations in manuscript transmission.

### Conclusion

The study of Hebrew accents in the Old Testament reveals fascinating differences between prose and poetic texts. By examining these patterns, scholars and readers can gain deeper insights into the structure, meaning, and literary style of various biblical books. Understanding these nuances enhances appreciation for the rich linguistic and literary heritage of the Hebrew Bible.
