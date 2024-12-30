# MOA Quizz command line

This command line is a simple quizz to test your abilities to calculate MOA.

## Usage

`moa_quizz`

## Example

```shell
moa_quizz -m moa
```

## Options

- `-m` or `--mode` : The mode of the quizz. Can be `moa` or `drop`, this argument is mandatory. [possible values: moa, drop, random]
- `-t` or `--tolerance`: The tolerance of the quizz. Default is `0.05`, 5%.
- `-n` or `--number-of-questions`: The number of questions of the quizz. Default is `10`.

## How it works

The quizz will ask you to calculate the MOA or the drop for a given distance and drop or MOA.
