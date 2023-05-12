<h2 align="center"> sorting </h2>

![intro](video/intro.gif)


- [Quick start](#quick)
- [What can do?](#what)
- [New](#new)
- [Plan](#plan)


<h4 id="quick">Quick start</h4>

just download the release version of the system you want

**❗ In this version it must be run in the directory where it lies ❗**

<h4 id="what">What can do?</h4>

this sorter can sort files by category, and if there is a recurring file,<br>
it will add its number to it in parentheses

![sample](video/sample.gif)

You can configure it in the **config.toml** file 

What is before = is the name of the directory, and after it in [...], they should be in lowercase without a dot ( . )<br> 
and in single brackets ( ' ) and between them there should be a coma ( , ).

```toml
Word = ['docx', 'doc', 'odt', 'rtf', 'txt']
```

<h4 id="new">New</h4>



<h4 id="plan">Plan</h4>

- [x] 1.1.0 - add multiple files (this is when several files have the same name
    <br> and not to overwrite the file just add the number at the end as Texte(number).txt)

- [ ] 1.2.0 - add crossplatform for Winduos

- [ ] 1.3.0 - add background mode to work when something is changed in the directory

- [ ] 1.4.0 - add it to be able to sort files where it's needed and it doesn't matter where it is

