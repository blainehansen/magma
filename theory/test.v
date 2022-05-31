Declare ML Module "magmide.plugin".
Require Import String.
Open Scope string_scope.
Require Import ZArith.
Open Scope Z_scope.


Inductive Instruction :=
	| InstExit
	| InstMov (src: nat) (dest: nat)
	| InstAdd (val: nat) (dest: nat)
.

Inductive Ast :=
	| Add (a: Ast) (b: Ast)
	| Number (a: Z)
.

MagmideParse "(4 + 4)" as four.
(* Check four. *)
(* Print four. *)

Magmide "something.mg" use yo.
Theorem yo_true: yo = true.
Proof. reflexivity. Qed.

MagmideInspectExpr (true).
MagmideInspectExpr (0).
MagmideInspectExpr ("yo").
MagmideInspectExpr (InstMov 0 1).
(* MagmideInspectExpr (Number 2). *)
MagmideInspectExpr (Add (Number 2) (Number 3)).
