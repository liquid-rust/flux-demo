---
marp: true
theme: default
---

# `Flux`: Liquid types for Rust

---

## Motivation

_Types vs. Floyd-Hoare logic_

## Demonstration
Flux - Liquid Types for Rust

## Evaluation
Flux v. Prusti for Memory safety

---

# Types vs. Floyd-Hoare logic

---

## Liquid/Refinements 101

```haskell
type Nat = {v: Int | 0 <= v}
```

* `Int` is the _base type_ of the value
* `v` names the _value_ being described
* `0 <=v` is a _predicate_ constraint

---

## Liquid/Refinements 101

Generate the sequence of values between `lo` and `hi`

```haskell
range :: lo:Int -> {hi:Int | lo <= hi} -> List {v:Int|lo <= v < hi}
```

---

## Liquid/Refinements 101

Generate the sequence of values between `lo` and `hi`

```haskell
range :: lo:Int -> {hi:Int | lo <= hi} -> List {v:Int|lo <= v < hi}
```

#### Input Type is a Precondition

`lo <= hi`

---

## Liquid/Refinements 101

Generate the sequence of values between `lo` and `hi`

```haskell
range :: lo:Int -> {hi:Int | lo <= hi} -> List {v:Int|lo <= v < hi}
```

#### Output Type is a Postcondition

_Every element_ in sequence is between `lo` and `hi`

---

# Types vs. Floyd-Hoare logic

Types _decompose_ assertions to _quantif-free_ refinements

---

## Lists in Dafny vs LiquidHaskell

<br>

![width:1100px](img/why_types_1_1.png)

---

## Accessing the `i`-th List Element

![width:1100px](img/why_types_1_2.png)


**Floyd-Hoare** Requires `elements` and _quantified_ axioms

**Liquid** Parametric _polymorphism_ yields spec for free

---
## Building and Using Lists

![width:1000px](img/why_types_1_3.png)

**Floyd-Hoare** _Quantified_ postcondition (hard to infer)

---
## Building and Using Lists

![width:1000px](img/why_types_1_3.png)

**Liquid** _Quantifier-free_ type (easy to infer)

```haskell
Int -> k:Int -> List {v:Int| k <= v}
```

---

# Types vs. Floyd-Hoare logic

Types decompose assertions to quantif-free refinements

... but what about **imperative programs**

---

## Motivation

Types vs. Floyd-Hoare logic

## Demonstration

_`Flux` Liquid Types for Rust_

## Evaluation
`Flux` v. `Prusti` for Memory Safety

---

## `Flux` Liquid Types for Rust

`flux` (/flʌks/)

n. 1 a flowing or flow. 2 a substance used to refine metals. v. 3 to melt; make fluid.


---

## `Flux` Liquid Types for Rust

1. [`basics`](src/basics.rs)

2. [`borrows`](src/borrows.rs)

3. [`ref-vec`](src/rvec.rs)

4. [`vectors`](src/vectors.rs)

---

## Motivation

Types vs. Floyd-Hoare logic

## Demonstration

_`Flux` Liquid Types for Rust_

## Evaluation

`Flux` v. `Prusti` for Memory Safety

---

## Evaluation

`Flux` v. `Prusti` for Memory Safety

---

## `Flux` v. `Prusti` : By the numbers

![width:800px](img/flux-v-prusti.png)

---

## `Flux` v. `Prusti` : Types Simplify Specifications

```rust
// Rust
fn store(&mut self, idx: usize, value: T)

// Flux
fn store(self: &mut RVec<T>[@n], idx: usize{idx < n}, value: T)

// Prusti
requires(index < self.len())
ensures(self.len() == old(self.len()))
ensures(forall(|i:usize| (i < self.len() && i != index) ==>
                    self.lookup(i) < old(self.lookup(i))))
ensures(self.lookup(index) == value)
```

**Quantifiers make SMT _slow_!**

---

## `Flux` v. `Prusti` : Types Enable Code Reuse

- `flux-kmeans.rs` vs. `prusti-kmeans.rs`

- polymorphism => API composition (`RVec.rs` and `RMat.rs`)

TODO -- vimdiff

---

## `Flux` v. `Prusti` : Types Simplify Invariants & Inference

- easy length invariant (due to `RVec::set` and `fft`)

- polymorphism => quantifier-free invariants (`kmp.rs`)

TODO -- vimdiff

---

## Motivation

Types vs. Floyd-Hoare logic

## Demonstration

_`Flux` Liquid Types for Rust_

## Evaluation

`Flux` v. `Prusti` for Memory Safety

---

## Conclusions

**Refinements + Rust's Ownership = Ergonomic Imperative Verification...**

* Specify complex invariants by _composing_ type constructors & QF refinements

* Verify complex invariants by _decomposing_ validity checks via syntactic subtyping

---

## Conclusions

**Refinements + Rust's Ownership = Ergonomic Imperative Verification...**

* Specify complex invariants by _composing_ type constructors & QF refinements

* Verify complex invariants by _decomposing_ validity checks via syntactic subtyping


**... But this is just the beginning**

* `Flux` restricts specifications, `Prusti` allows _way_ more ...

* ... how to stretch types to "full functional correctness"?

---

# `flux` https://github.com/liquid-rust/flux/