  type 'a t =
    | Nil
    | Node of 'a * 'a t
  
  let rec read index = function
    | Nil -> None
    | Node (head, tail) -> if index = 0 then Some(head) else read (index - 1) tail
