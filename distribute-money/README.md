# distribute-money

Calculates transactions within a group to balance their expenses and settle all outstanding balances.

## Installation

```
cargo install --git https://github.com/vmiklos/vmexam distribute-money
```

## Usage

Assuming you already used a spreadsheet to calculate who is a creditor (spent more money than
necessary) and a debtor (spent less money than necessary) in the group, you can build an input CSV
like the one in `src/fixtures/input.csv`. Then you can run:

```
cargo run -- src/fixtures/input.csv
```

And you'll get a list of suggested transactions, so at the end nobody is a creditor or debtor
anymore.

Note that this is a bit more complex than just assuming you pay e.g. 20% of the total for a group of
5 people, this handles the use-case when e.g. you have 5 families and one family creates larger cost
than an other family.
