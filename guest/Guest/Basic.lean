
partial def sum (n : UInt32) : UInt32 :=
  if n == 0 then 0 else (n + sum (n - 1)) &&& 0xFFFF
