# Pealn

**Pealn** is a Rust library for **printing coloured text** to make your CLI app  beautiful as a Peacock

## Features

- **Easy to use** Apply modification inside string
- **Pre defined Colors** Common colors  are already defined
- **Use RGB colors** Apply colors using RGB value

## Installation

Add Pealn to your Cargo.toml:

```toml
[dependencies]
pealn = "0.1"
```

Add Peacock using Cargo CLI:
```
cargo add Pealn
```
Available macros to print colored and styled text

```rust
pea!("hello world"); // to print on same line
pealn!("hello world"); // to print on next line
```


print new line  with colored and styles
## Format

```rust
[foreground,background,styles....](text) 
```

### Available Colors 
 red, green, blue, yellow, cyan, magenta, black, white
  
### Available Styles 
 bold, dim, italic, underline, blink, reverse, hidden, strikethrough

 
## Examples
 
 To print text with foreground
 ```rust
 use pealn::{pealn};
 pealn!("[yellow](Hello) [green](World)!");
 let name  = "Subham Shaw";
 pealn!("[yellow,bold](Name) : [bold,hidden]({}) " , name );
 ```

 you can use RGB color 

```rust
 use pealn::{pealn};
 pealn!("[(25,45,78)](Hello) [(34,67,78)](World)!");
 ```

 To print text with foreground and background
 ```rust
 use pealn::{pealn};
 pealn!("[yellow,white](Hello) [green,white](World)!");
 ```
 
 To print text with styles
 ```rust
 use pealn::{pealn};
 
 pealn!("[bold,underline](Hello) [italic](World)!");
 ```
 
 To print text with color and styles
 ```rust
 use pealn::{pealn};
 //here order of colors and styles does not matter, 
 //first color will be used as foreground and second as background
 pealn!("[red,green,bold,underline](Hello) [yellow,white,italic](World)!");
 ```
---

*Peacock makes your CLI apps more colorful and expressive!*