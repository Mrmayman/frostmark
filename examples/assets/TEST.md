A test of <mark>everything</mark> this currently supports.

<center>

# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

Normal Text.

Normal, *italic*, **bold**, ~~strikethrough~~, __underline__, `code`, [link](https://example.com)

![image](https://github.com/Mrmayman/quantumlauncher/raw/main/assets/icon/ql_logo.png)

Image as <img src="https://github.com/Mrmayman/quantumlauncher/raw/main/assets/icon/ql_logo.png" width="50">

</center>

- [Link that points to something](https://example.com)
- [Link that doesn't point anywhere]()
- Link with image: [![image](https://github.com/Mrmayman/quantumlauncher/raw/main/assets/icon/ql_logo.png)]()

> Block Quote.
>
> - Point 1
> - Point 2
>
> > Nested Block Quote
> >
> > ```
> > #include <cstdio>
> >
> > int main() {
> >     std::printf("Hello, World!\n");
> > }
> > ```


---

Some space...
<br><br>
...between these lines.
<hr>

Tasks:
- [ ] Do thing 1
- [ ] Do thing 2
- [x] Do thing 3

List:
1. Nesting ordered list
    1. Subitem 1
    2. Subitem 2
    3. Subitem 3
2. Nesting unordered list
    - Subitem 1
    - Subitem 2
    - Subitem 3
3. Deep nesting
    1. Subitem
        - Subitem
          - Subitem
    2. Subitem
        1. Subitem

<details>
<summary>Summary (shown)</summary>
Text inside
<summary>Not shown in preview</summary>
</details>

<details>
    <summary>Ruby tests</summary>
    <ruby>東<rt>とう</rt>京<rt>きょう</rt></ruby> renders as 東京 と う き ょ う<br>
    <ruby>漢字<rp>(</rp><rt>かんじ</rt><rp>)</rp></ruby> renders as 漢字(かんじ)<br>
    <br>Edge cases:<br>
    <ruby> <rt>かん</rt> 漢 </ruby><br>
    <ruby> 漢 text <rt>かん</rt> </ruby><br>
    <ruby>漢 字 <rt>かん</rt> <rt>じ</rt> </ruby><br>
    <ruby> <span> 漢<rt>かん</rt> </span></ruby>
</details>
