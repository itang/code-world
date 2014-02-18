SHELL TIPS (as H1)
==================

## Blockquotes
> linux shell tips collects
> > for remember!
>
> 1. oneline
> 2. twoline
>
> some samples
>
>     println!("hello,world")

## Lists
### order list

1. Bird
2. McHale
3. Parish
### unorder
* one
* two
* three

as 

+ Red
+ Green
+ Blue

as

- good
- ok
- yes

as more

* longlong  
  longlong

  somesome.

* shortshort  
  shortshort

* A list item with a blockquote:

    > This is a blockquote
    > inside a list item.

* with code block with 8 space:

        assert_eq!(1, 1);

## Code block
code 1

    fn main() {
        println!("Hello");
    }

html

    <div class="footer">
        &copy; 2004 Foo Corporation
    </div>

## BR
s0
* * *
s1
***
s2
******
s3
- - -
s4
- - - - - -

## Links

### Inline
- This is [an example](http://example.com/ "Title") inline link.
- [this link](http://example.net/) has no title attribte.

### reference-style

- This is [an example][id] reference-style link.
- this is [an example] [id] reference-style link.
- Visit [Daring Fireball][] for more information.

[id]: http://example.com/ "Optional Title Here"
[Daring Fireball]: http://example.com/ (some title)

## Strong

common world

\*this text is surrounded by literal asterisks\*

### use *

*hello,world*

**hello,world**

***hello,world***

### use _

_hello,world_

__hello,world__

## Code

use `println!` output to stdout.

A single backtick in a code span: `` ` ``

A backtick-delimited string in a code span: `` `foo` ``

## Images

![Alt text](/path/to/image.jpg)

![Alt text](/path/to/image.jpg "Option title")

![Alt text][imgid]

[imgid]: url/to/image "Optional title attribute"

## Other

auto links

<http://example.com/>

to:

    <a href="http://example.com/">http://example.com/</a>

<address@example.com>

TIPS-1
------------
- *check cmd exists*

        if type -p curl >/dev/null 2>&1; then
        fi

- ss

## TIPS-2

COMENTS (as H2)
---------------

# Other (as H1)