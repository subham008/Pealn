# Introduction
**Pealn** is a **Rust library** to print **colored** and **styled** to **console**
## Features
- **Fatest** Pealn is fastest among other famous ones. see [benchmarks](.././benches/README.md)
- **Compile time** format pealn codes during compile time 
- **Easy to use** Apply modification inside string
- **Pre defined Colors** Common colors  are already defined
- **Text Styles** Add Styles like , bold , italic , underline and more
- **Use RGB colors** Apply colors using RGB value

# Installation
Add Pealn to your Cargo.toml , see [crates.io/Pealn](https://crates.io/crates/pealn) for latest version

```toml
[dependencies]
pealn = "0.3"
```

Add Pealn using Cargo CLI:
```
cargo add Pealn
```

# How to use it
## Format

```rust
[foreground,background,styles....](text) 
```
> *Note: the order of foreground , background and styles could be anything*

## Understand By exmaples
**Lets add forground color to a text using** 
```rust
pealn!("[green](John Doe)") 
```
output
> <span style="color:green">John Doe</span>

**Lets  print more text with diffrent foreground colors**
```rust
pealn!("Name : [green](John Doe) \n Age : [yellow](32)") 
```
output
> Name : <span style="color:green">John Doe</span>  
Age : <span style="color:orange">32</span>

**Now lets add  both foreground and background color**
```rust
pealn!("Name : [green,yellow](John Doe) \n Age : [yellow](32)") 
```
output
>Name: <span style="background-color: yellow; color: green; padding: 2px 6px; border-radius: 4px;">John Doe</span>  
Age : <span style="color:orange">32</span>

### *Note first color will be considered as foreground and next color will be consider as background , you cannot add more than 2 color 

**Now lets change only bacground of `John Doe` of Name not foreground**

To do it you have to define first color or  foreground color as Default
```rust
pealn!("Name : [default,yellow](John Doe) \n Age : [yellow](32)") 
```
output
>Name: <span style="background-color: yellow; padding: 2px 6px; border-radius: 4px;">John Doe</span>  
Age : <span style="color:orange">32</span>

## 🎨 Available Predefined Colors

| Name      | Preview                          | Example Code         |
|-----------|----------------------------------|----------------------|
| Red       | <span style="color:#ff0000;">■</span> | `[red](text)`        |
| Green     | <span style="color:#00ff00;">■</span> | `[green](text)`      |
| Blue      | <span style="color:#0000ff;">■</span> | `[blue](text)`       |
| Yellow    | <span style="color:#ffff00;">■</span> | `[yellow](text)`     |
| Cyan      | <span style="color:#00ffff;">■</span> | `[cyan](text)`       |
| Purple    | <span style="color:#800080;">■</span> | `[purple](text)`     |
| Magenta   | <span style="color:#ff00ff;">■</span> | `[magenta](text)`    |
| Black     | <span style="color:#000000;">■</span> | `[black](text)`      |
| White     | <span style="color:#ffffff;">■</span> | `[white](text)`      |

> *Note: Color preview may not render*

### So didnt found your choice on predefined table above No problem you can use `RGB` colors
```rust
[(r,g,b),background,styles....](text) 
```

**Lets use RGB to give any color to your text**

To use RGB use `(r,g,b)` instead of predefined color

```rust
pealn!("[(190, 3, 252)](John Doe)") 
```
output
> <span style="color: rgb(190, 3, 252);">John Doe</span>

## Adding `Styles` to your Text
You can add multiple styles but styles should not be repeated

**lets print name in bold**

```rust
pealn!("[bold](John Doe)") 
```
output
> <span style="font-weight: bold;">John Doe</span>

**lets print name in bold and italic**

```rust
pealn!("[bold,italic](John Doe)") 
```
output
> <span style="font-weight: bold; font-style: italic;">John Doe</span>

**lets print name in bold and italic with yellow foreground**

```rust
pealn!("[yellow,bold,italic](John Doe)") 
```
output
> <span style="font-weight: bold; font-style: italic; color: yellow;">John Doe</span>

**lets print name in bold and italic with yellow foreground and blue background**

```rust
pealn!("[yellow,blue,bold,italic](John Doe)") 
```
output
> <span style="font-weight: bold; font-style: italic; color: yellow; background-color: blue;">John Doe</span>

## ✨ Available Styles

| Style          | Example Code         | Description                  |
|----------------|---------------------|------------------------------|
| Bold           | `[bold](text)`      | **Bold text**                |
| Dim            | `[dim](text)`       | Dim/faint text               |
| Italic         | `[italic](text)`    | *Italic text*                |
| Underline      | `[underline](text)` | <u>Underlined text</u>       |
| Blink          | `[blink](text)`     | Blinking text (not always supported) |
| Reverse        | `[reverse](text)`   | Reverse video                |
| Hidden         | `[hidden](text)`    | Hidden text                  |
| Strikethrough  | `[strikethrough](text)` | ~~Strikethrough~~           |

> *Some styles may not be supported in all terminals.*

## Pealn is an Alternative
Pealn has alternative of below macros , their argument types , working all are same  as traditional , but with a pealn  color and style formatting
| Traditional Rust Macro | Pealn Macro Alternative|
|------------------------|------------------------|
| `print!()`             | `pea!()`               |
| `println!()`           | `pealn!()`             |
| `write!()`             | `pealn_write!()`       |
| `writeln!()`           | `pealn_writeln!()`     |
| `format!()`            | `pealn_format!()`      |
| `eprint!()`            | `pealn_eprint!()`      |
| `eprintln!()`          | `pealn_eprintln!()`    |
