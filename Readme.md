# lingo-rs

lingo-rs allows you to generate a graph depicting the programming language distribution in a git repo over time. 

![Example output graph](https://user-images.githubusercontent.com/2777107/154803382-9975cb48-33d0-4659-a8cc-52b3afa517a0.png)

## Getting started

To create your first graph, run the following:

```sh
lingo-rs "C:/source/my-repo" --name "Example" --start "2019-01-01"
```

Other configurations include:

* `--end` to specify the end date (default: today)
* `--branch` to specify the branch you want to use (default: master)

Notes:
* Make sure that your repo has no uncommitted changes
* At the end of the run, lingo-rs will bring you back to HEAD of your branch. However if you cancel it earlier, it will end up with a detached HEAD
* If your git history contains problematic files such as case-conflicting files on a case-sensitive folder, the repo can end up in a bad state when that commit is checked out.