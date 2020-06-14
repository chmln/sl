# sl - symbolic links

Because `ln` is annoying and ambiguous. This is basically `ln -sT` with some nice improvements.

## Features

- paths are canonicalized
- the symbolic link is created along with any parent directories, if necessary
- Only one way to create a symlink
  - yeah, this is a feature because `ln` has 4 ways of doing it
