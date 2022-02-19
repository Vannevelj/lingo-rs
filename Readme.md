# lingo-rs

lingo-rs allows you to generate a graph depicting the programming language distribution in a git repo over time. 

## Getting started

To create your first graph, run the following:

```sh
lingo-rs "C:/source/my-repo" --name "Example" --start "2019-01-01"
```

Other configurations include:

* `--end` to specify the end date (default: today)
* `--branch` to specify the branch you want to use (default: master)