```
for i in 0..vec.len() {
    if i == 3 {
        return vec[i];
    } else {
        -1
    }
}
```


```
let res = match s {
    Ok(content) => content,
    Err(err) => panic!("something went wrong.\n Reason: {}", err.to_string())
}
```

```
pub struct ParserState {
    pub input_lines: Vec<String>,
    pub token: Token,
    pub current_line: (usize, String),
    pub code_blocks: Vec<usize>,
    pub next_ul: usize,
}
```