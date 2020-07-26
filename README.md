# rslog

Prolog-like logic language implemented in Rust.

## Usage
It is basically same as [swipl](https://www.swi-prolog.org/).

`rules.pl`
```prolog
male(kobo).
male(koji).
female(sanae).
parent(kobo, koji).
parent(kobo, sanae).
parent(koji, iwao).
ancestor(X,Y) :- ancestor(Z,Y), parent(X,Z).
ancestor(X,Y) :- parent(X,Y).

father(X, Y) :- parent(X, Y), male(Y).
mother(X, Y) :- parent(X, Y), female(Y).
```

```
$ cargo run
...
?- ['rules.pl'].
true.

?- father(kobo, X).
X = koji.
true.

?- mother(kobo, sanae).
true.

?- ancestor(kobo, X).
X = koji.
X = sanae.
X = iwao.
true.
```

## How is this different from Prolog?
- Uses BFS for searching solutions instead of DFS, which avoids unnecessary infinity loop.
- Evaluates rules regardless of the order, which avoids unnecessary infinity loop.
- Performs occurence check in unification.

## TODO
- As in [swipl](https://www.swi-prolog.org/), ask user to continue search or not after finding a solution.
- Implement Cut and Negation.
- Implement built in functions and list syntax.
