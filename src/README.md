# iamlisp

Rust implementation of iamlisp with iterative evaluation of program.

## Progress

### Data structures

- [x] Scalar literals:
  - [x] `Int64`
  - [x] `Float64`
  - [x] `Boolean`
  - [x] `String`
  - [ ] `Nil`
- [ ] `Set`
- [ ] `Map`
- [ ] `Vector`

### Language features

- [ ] Math operations: `-, +, *, /, //, %, pow, sqrt, max, min`
- [ ] Logic operations: `>, <, >=, <=, =, !=, !, !!`
- [x] Keyword `def`
- [x] Keyword `quote`
- [x] Keyword `lambda`
- [x] Keyword `macro`
- [x] Keyword `cond`
- [ ] Keyword `loop`
- [ ] Keyword `defun`
- [ ] Keyword `defmacro`
- [ ] Keyword `macroexpand`
- [ ] Methods for `List`, `Vector` manipulation: `map, filter, reduce, find, includes?, some, any`
- [ ] Methods for `Set`, `Map` manipulation: `add, has, delete`
- [ ] Lambda arguments destructuring
- [ ] Tail call optimization


## Syntax examples

### Define variable

```
(def a 10 b 20)
(def foo "Hello World")
(def bar true)
```

### Define function (WIP)

```
(defun sum (a b) (+ a b))
```

### Define macro (WIP)

```
(defmacro backwards (. body) (eval (cons 'begin (.reverse 'body))))
```

### Define lambda

```
(def my-lambda (lambda (a b) (+ a b)))
```

### Iterative loop (WIP)

```
; Print numbers from 100 to 0

(loop (x 100)
      (print x)
      (cond ((> x 0) (recur (dec x)))))

; Fibonacci using iterative loop

(defun fib (n)
  (loop (x 0 y 1 i n)
    (cond ((<= i 0) x) ((recur y (+ x y) (dec i))))))
```

### Define variable using list destructuring (WIP)

```
; Nested destructuring
(def (a (b c)) '(2 '(4 6)))

; Destructuring with rest
(def (first . rest) '(1 2 3 4 5))
```
