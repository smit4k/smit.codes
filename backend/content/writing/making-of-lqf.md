---
title: The Making of LQF
date: 2025-07-26
tags: [configuration, language-design]
links: []
description:
  An overview of the design philosophy and development process behind LQF, my
  custom configuration language, including the reasoning, structure, and syntax
  decisions that shape how it works.
---

### How it Started

We all know JSON, YAML, and TOML. They're popular configuration languages.
They're great, don't get me wrong, but I wanted something different, something a
bit more simple, readable, uniform, and most importantly, something
_aesthetically pleasing_. I also had a lot of time to spend, as it was the end
of the school year and all of my exams were over, so this was the perfect time
to make my own configuration language, LQF, the Lightweight Quick Format,

### The Design

I wanted LQF to be extremely simple and revolve around a singular symbol. The
symbol of choice is one of my favorite symbols, the `>` (greater than operator).
It represents an arrow to the right, and it's very readable.

### The Basics

A configuration language needs to have **sections** and **assignments**

---

#### Sections

**Sections** begin with a `>` followed immediately by a section name.

Example:

```
> settings
```

---

#### Assignments

**Assignments** are key-value pairs within a section. You can assign using `>>`

Example:

```
username >> "smit4k"
active >> true
```

---

#### Comments

Another feature that configuration languages have are **comments**. **Comments**
in LQF start with `#` and continue to the end of the line.

Example:

```
# This is a comment
```

---

#### Supported Values

LQF supports a lot of values, when assigning a key to a value. Read the table
below for more info:

| Type    | Syntax Example            | Description                                           |
| ------- | ------------------------- | ----------------------------------------------------- |
| String  | `"Hello, world!"`         | Double-quoted text                                    |
| Number  | `42`, `3.14`              | Integers, Floating points                             |
| Boolean | `true`, `false`           | Logical values                                        |
| Null    | `null`                    | Null value                                            |
| Array   | `[1, 2, 3]`, `["a", "b"]` | Comma-separated list of values inside square brackets |

### That's It

That's the entire language, as of now. It's essentially completed. It's
**simple**, **readable**, and looks **awesome** (in my opinion). I'm really
happy with how it turned out.

If you would like to **contribute** to the language, you can check out the
specifications in my Github repo,
[smit4k/lqf-spec](https://github.com/smit4k/lqf-spec), all of the info about the
language is there.

Here's a full example file for you to enjoy

```
# Example LQF file
# Designed by smit4k (smit@smit.codes)

> server
host >> "localhost"
port >> 8080
use_ssl >> true

> database
engine >> "postgres"
host >> "db.internal"
port >> 5432
username >> "admin"
password >> "s3cr3t"
max_connections >> 20

> features
enabled >> ["search", "analytics", "dark_mode"]
beta >> false

> logging
level >> "info"
file >> "logs/app.log"

> cache
backend >> "redis"
ttl_seconds >> 3600
enabled >> true

> meta
version >> "1.0.0"
maintainers >> ["bob@example.com", "gus@example.com"]
```
