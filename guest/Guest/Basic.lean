
partial def sum (n : Nat) : Nat :=
  if n == 0 then 0 else (n + sum (n - 1)) &&& 0xFFFF
